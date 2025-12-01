<script lang="ts">
import type { ArrowLinkProps } from '$lib/types/components/arrowlink'
import { ChevronLeft, ChevronRight } from 'lucide-svelte'

let { children, href = '/', direction = 'left' }: ArrowLinkProps = $props()

let Icon = direction === 'left' ? ChevronLeft : ChevronRight
</script>

<a {href} class="backlink {direction}">
  <span class="icon">
    <Icon />
  </span>
  {@render children()}
</a>

<style lang="scss">
.backlink {
  font-size: var(--font-size-s);
  color: var(--text-muted);
  display: inline-flex;
  align-items: center;
  gap: var(--padding-xs);
  margin-bottom: var(--padding-s);

  &:hover {
    color: var(--text-primary);

    .icon {
      transform: translateX(-2px);
    }
  }

  &.right {
    flex-direction: row-reverse;

    &:hover {
      .icon {
        transform: translateX(2px);
      }
    }
  }

  &:active {
    color: var(--text-primary);
  }

  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
</style>
