<div class="logo-wrapper">
  <svg
    width="250"
    height="250"
    viewBox="0 0 250 250"
    fill="none"
    xmlns="http://www.w3.org/2000/svg">
    <circle class="slow" cx="160" cy="90" r="90" fill="var(--logo-upper)" />
    <circle class="slow" cx="90" cy="160" r="90" fill="var(--logo-lower)" />
    <mask
      id="mask0_1_12"
      style="mask-type:alpha"
      maskUnits="userSpaceOnUse"
      x="0"
      y="70"
      width="180"
      height="180">
      <circle cx="90" cy="160" r="90" fill="var(--logo-lower)" />
    </mask>
    <g mask="url(#mask0_1_12)">
      <circle cx="160" cy="90" r="90" fill="var(--logo-neutral)" />
    </g>
  </svg>
</div>

<style lang="scss">
.logo-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;

  svg {
    height: var(--logo-size, 1em);
    width: auto;

    .slow {
      transition: fill var(--animation-length-xl) var(--better-ease-out);
    }
  }
}
</style>
