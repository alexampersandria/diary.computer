<div class="footer">
  <div class="footer-sections">
    <div class="footer-section">
      <div class="footer-section-title">Legal</div>
      <a href="/page/terms">Terms of Service</a>
      <a href="/page/privacy">Privacy Policy</a>
    </div>

    <div class="footer-section">
      <div class="footer-section-title">Resources</div>
      <a href="/docs/">Documentation</a>
      <a
        href="https://github.com/alexampersandria/diary.computer"
        target="_blank"
        rel="noopener noreferrer">GitHub</a>
    </div>
  </div>
</div>

<style lang="scss">
.footer {
  padding-top: max(var(--padding-xl), 4vh);
  padding-bottom: max(var(--padding-xl), 10vh);

  .footer-sections {
    display: flex;
    justify-content: center;
    gap: var(--padding-xl);

    .footer-section {
      display: flex;
      flex-direction: column;
    }

    a {
      color: var(--text-muted);

      &:hover {
        color: var(--text-primary);
      }
    }

    @media screen and (max-width: 768px) {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--padding-m);
    }
  }
}
</style>
