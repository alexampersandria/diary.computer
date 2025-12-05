<script lang="ts">
import type { SessionProps } from '$lib/types/components/session'
import {
  Bot,
  Chrome,
  Clock,
  ClockCheck,
  Compass,
  Cpu,
  Gamepad,
  Globe,
  Laptop,
  Option,
  Smartphone,
  Tablet,
  Trash,
  Tv,
} from 'lucide-svelte'
import Button from './Button.svelte'
import { parseUserAgent } from '$lib/useragent/useragent'
import { formatTimestamp, timeAgo } from '$lib/utils/time'
import Alert from './Alert.svelte'

let { session, active = false, onrevoke }: SessionProps = $props()

let userAgent = $derived.by(() => parseUserAgent(session.user_agent))
let DeviceIcon = $derived.by(() => {
  if (userAgent.isBot) {
    return Bot
  } else if (userAgent.isMobile) {
    return Smartphone
  } else if (userAgent.isTablet) {
    return Tablet
  } else if (userAgent.isChrome) {
    return Chrome
  } else if (userAgent.isSafari) {
    return Compass
  } else if (userAgent.isMacOS) {
    return Option
  } else if (userAgent.isTV) {
    return Tv
  } else if (userAgent.isConsole) {
    return Gamepad
  } else if (userAgent.isDesktop) {
    return Laptop
  } else {
    return Cpu
  }
})

let revokeLoading = $state(false)
let revokeError = $state(false)
const handleRevoke = () => {
  if (onrevoke) {
    revokeError = false
    revokeLoading = true
    onrevoke(session.id).then(revoked => {
      revokeLoading = false
      revokeError = !revoked
    })
  }
}

let timestamp = $derived.by(() => formatTimestamp(session.accessed_at))
</script>

<div class="session-wrapper">
  <div
    class="session"
    class:active
    class:error={revokeError}
    class:loading={revokeLoading}>
    <div
      class="session-item device"
      title={`User Agent: ${userAgent.useragent}`}>
      <div class="icon">
        <DeviceIcon />
      </div>
      <div class="value">
        {userAgent.display}
      </div>
    </div>
    <div
      class="session-item ip-address"
      title={`IP Address: ${session.ip_address}`}>
      <div class="icon">
        <Globe />
      </div>
      <div class="value" aria-label={`IP Address: ${session.ip_address}`}>
        {session.ip_address}
      </div>
    </div>
    <div class="session-item last-active" title={`Last Active: ${timestamp}`}>
      {#if active}
        <div class="icon">
          <ClockCheck />
        </div>
        <div class="value" aria-label="This device, last active">
          This device
        </div>
      {:else}
        <div class="icon">
          <Clock />
        </div>
        <div class="value" aria-label={`Last Active: ${timestamp}`}>
          {timeAgo(session.accessed_at)}
        </div>
      {/if}
    </div>
    {#if onrevoke}
      <div class="revoke">
        <Button
          fullwidth
          onclick={handleRevoke}
          loading={revokeLoading}
          disabled={active}
          aria-label="Revoke Session">
          <Trash />
        </Button>
      </div>
    {/if}
  </div>

  {#if revokeError}
    <Alert solid type="error">
      Failed to revoke session
      {#snippet actions()}
        <Button onclick={handleRevoke} type="ghost">Try again</Button>
        <Button onclick={() => (revokeError = false)}>Dismiss</Button>
      {/snippet}
    </Alert>
  {/if}
</div>

<style lang="scss">
.session-wrapper {
  &:has(:global(.alert)) {
    .session {
      border-bottom-left-radius: 0;
      border-bottom-right-radius: 0;
      border-bottom: none;
    }

    :global(.alert) {
      border-top-left-radius: 0;
      border-top-right-radius: 0;
    }
  }

  .session {
    display: grid;

    grid-template-columns: 2fr 1fr 1fr;
    &:has(.revoke) {
      grid-template-columns: 2fr 1fr 1fr 4rem;
    }

    @media (max-width: 768px) {
      &:has(.revoke),
      & {
        grid-template-columns: 1fr;
      }
      gap: var(--padding-s);
    }

    align-items: center;
    padding: var(--padding-s);
    border: var(--border-width) solid var(--border-color);
    border-radius: var(--radius-s);
    transition: var(--interactive-transition);

    .icon {
      color: var(--text-muted);
    }

    &.active {
      background-color: var(--color-info-background);
      border-color: var(--color-info-border);

      .icon {
        color: var(--color-info);
      }
    }

    &.error {
      background-color: var(--color-error-background);
      border-color: var(--color-error-border);

      .icon {
        color: var(--color-error);
      }
    }

    &.loading {
      background-color: var(--background-secondary);
      color: var(--text-muted);

      .icon {
        color: var(--text-dimmed);
      }
    }

    .session-item {
      display: flex;
      align-items: center;
      gap: var(--padding-s);
      overflow: hidden;
      padding-right: var(--padding-s);

      .value {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }

    .revoke {
      display: flex;
      justify-content: flex-end;
    }
  }
}
</style>
