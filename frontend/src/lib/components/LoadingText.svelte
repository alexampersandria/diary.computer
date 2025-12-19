<script lang="ts">
import type { LoadingTextProps } from '$lib/types/components/loadingtext'

let { length = 10 }: LoadingTextProps = $props()
</script>

<div class="loading-text" style="--length: {length}"></div>

<style lang="scss">
.loading-text {
  display: inline-block;
  width: calc(var(--length) * 1ch);
  height: 1em;
  background: linear-gradient(
    135deg,
    var(--skeleton-loading-background) 25%,
    var(--skeleton-loading-highlight) 50%,
    var(--skeleton-loading-background) 75%
  );
  background-size: 300% 100%;
  border-radius: var(--radius-xxs);
  animation: loading-shimmer calc(var(--animation-length-xxl) * 2) infinite
    linear;

  @keyframes loading-shimmer {
    0% {
      background-position: -300% 0;
    }
    100% {
      background-position: 300% 0;
    }
  }
}
</style>
