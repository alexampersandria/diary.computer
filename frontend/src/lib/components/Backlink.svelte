<script lang="ts">
import { ChevronLeft } from 'lucide-svelte'

let { children, href = '/' } = $props()
</script>

<a {href} class="backlink">
  <div class="icon">
    <ChevronLeft />
  </div>
  {@render children()}
</a>

<style lang="scss">
.backlink {
  font-size: var(--font-size-s);
  color: var(--text-muted);
  display: inline-flex;
  align-items: center;
  gap: var(--padding-xs);
  padding-bottom: var(--padding-s);

  &:hover {
    color: var(--text-primary);

    .icon {
      transform: translateX(-2px);
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
