<script lang="ts">
import type { LabelProps } from '$lib/types/components/label'

let {
  children,
  for: forProp,
  weight = 'normal',
  size = 'medium',
}: LabelProps = $props()
</script>

{#if forProp}
  <label for={forProp} class="label {weight} {size}">
    {@render children()}
  </label>
{:else}
  <div class="label {weight} {size}">
    {@render children()}
  </div>
{/if}

<style lang="scss">
.label {
  display: inline-flex;
  align-items: center;
  gap: var(--padding-s);

  :global(.lucide-icon) {
    font-size: var(--font-size-xl);
    color: var(--text-muted);
  }

  &.bold {
    font-weight: 600;
  }

  &.small {
    font-size: var(--font-size-s);
  }

  &.medium {
    font-size: var(--font-size-m);
  }

  &.large {
    font-size: var(--font-size-l);
  }
}
</style>
