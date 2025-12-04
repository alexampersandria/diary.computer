<script lang="ts">
import type { SessionProps } from '$lib/types/components/session'
import {
  Bot,
  Clock,
  ClockCheck,
  Globe,
  Laptop,
  Option,
  Smartphone,
  Tablet,
  Trash,
} from 'lucide-svelte'
import Button from './Button.svelte'
import { parseUserAgent } from '$lib/utils/userAgent'
import { formatTimestamp, timeAgo } from '$lib/utils/time'
import Alert from './Alert.svelte'

let { session, active = false, onrevoke }: SessionProps = $props()

let userAgent = $derived.by(() => parseUserAgent(session.user_agent))
let DeviceIcon = $derived.by(() => {
  if (userAgent.isMacOS) {
    return Option
  } else if (userAgent.isNonHuman) {
    return Bot
  } else if (userAgent.isMobile) {
    return Smartphone
  } else if (userAgent.isTablet) {
    return Tablet
  } else {
    return Laptop
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
</script>

<div class="session-wrapper">
  <div class="session" class:active class:error={revokeError}>
    <div class="session-item device">
      <div class="icon">
        <DeviceIcon />
      </div>
      <div class="value" title={userAgent.raw}>
        {userAgent.display}
      </div>
    </div>
    <div class="session-item ip-address">
      <div class="icon">
        <Globe />
      </div>
      <div class="value">
        {session.ip_address}
      </div>
    </div>
    <div class="session-item last-active">
      {#if active}
        <div class="icon">
          <ClockCheck />
        </div>
        <div class="value" title={formatTimestamp(session.accessed_at)}>
          This device
        </div>
      {:else}
        <div class="icon">
          <Clock />
        </div>
        <div class="value" title={formatTimestamp(session.accessed_at)}>
          Last active {timeAgo(session.accessed_at)}
        </div>
      {/if}
    </div>
    {#if onrevoke}
      <div class="revoke">
        <Button
          type="ghost"
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

    grid-template-columns: 1fr 1fr 1fr;
    &:has(.revoke) {
      grid-template-columns: 1fr 1fr 1fr 4rem;
    }

    align-items: center;
    padding: var(--padding-s);
    border: var(--border-width) solid var(--border-color);
    border-radius: var(--radius-s);

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
