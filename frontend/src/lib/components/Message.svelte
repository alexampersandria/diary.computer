<script lang="ts">
import type { MessageProps } from '$lib/types/components/message'
import {
  CircleAlert,
  CircleCheck,
  InfoIcon,
  TriangleAlert,
} from 'lucide-svelte'

let {
  children,
  type = 'info',
  colortext,
  size = 'medium',
  muted,
  centered,
}: MessageProps = $props()

let MessageIcon = $derived.by(() => {
  switch (type) {
    case 'error':
      return CircleAlert
    case 'warning':
      return TriangleAlert
    case 'success':
      return CircleCheck
    case 'info':
      return InfoIcon
  }
})
</script>

<div class="message {type} {size}" class:colortext class:muted class:centered>
  <div class="message-icon lucide-icon-wrapper">
    <MessageIcon />
  </div>
  <div class="message-content">
    {@render children()}
  </div>
</div>

<style lang="scss">
.message {
  display: flex;
  align-items: flex-start;

  &.centered {
    justify-content: center;
  }

  &.small {
    font-size: var(--font-size-s);
    gap: var(--message-gap-s);
  }

  &.medium {
    font-size: var(--font-size-m);
    gap: var(--message-gap-m);
  }

  &.large {
    font-size: var(--font-size-l);
    gap: var(--message-gap-l);
  }

  &.error {
    .message-icon {
      color: var(--color-error);
    }

    &.colortext {
      color: var(--color-error);
    }
  }

  &.warning {
    .message-icon {
      color: var(--color-warning);
    }

    &.colortext {
      color: var(--color-warning);
    }
  }

  &.success {
    .message-icon {
      color: var(--color-success);
    }

    &.colortext {
      color: var(--color-success);
    }
  }

  &.info {
    .message-icon {
      color: var(--color-info);
    }

    &.colortext {
      color: var(--color-info);
    }
  }

  &.muted {
    .message-icon {
      color: inherit;
    }
  }
}
</style>
