<script lang="ts">
import { goto } from '$app/navigation'
import {
  Book,
  CalendarDays,
  ChartLine,
  FolderOpen,
  Github,
  House,
  LogOut,
  PanelLeftClose,
  PanelLeftOpen,
  ScrollText,
  Settings,
  Tags,
  User,
  UserCog,
} from 'lucide-svelte'

import Button from '$lib/components/Button.svelte'
import Modal from '$lib/components/Modal.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import Label from '$lib/components/Label.svelte'
import Logo from '$lib/components/Logo.svelte'
import Calendar from '$lib/components/Calendar.svelte'
import { page } from '$app/state'
import { watch } from 'runed'
import ColorPicker from '$lib/components/ColorPicker.svelte'
import ThemePicker from '$lib/components/ThemePicker.svelte'

let { children } = $props()

let userStore = useUserStore()
let uiStore = useUiStore()

let isDragging = $state(false)
let userDetailsModal = $state(false)
let settingsModal = $state(false)
let leftMenuOpenMobile = $state(false)

$effect(() => {
  if (!userStore.sessionId && !uiStore.loading) {
    goto('/')
  }
})

const startDrag = () => {
  isDragging = true

  const onMouseMove = (event: MouseEvent) => {
    uiStore.leftMenuWidth = event.clientX
  }

  const onMouseUp = () => {
    isDragging = false
    window.removeEventListener('mousemove', onMouseMove)
    window.removeEventListener('mouseup', onMouseUp)
  }

  window.addEventListener('mousemove', onMouseMove)
  window.addEventListener('mouseup', onMouseUp)
}

const toggleLeftMenu = () => {
  uiStore.leftMenuOpen = !uiStore.leftMenuOpen
}

const toggleLeftMenuMobile = () => {
  leftMenuOpenMobile = !leftMenuOpenMobile
}

const handleLogout = () => {
  userDetailsModal = false
  userStore.logOut()
}

watch(
  () => page.url.pathname,
  (to, from) => {
    if (to && to !== from) {
      leftMenuOpenMobile = false
    }
  },
)
</script>

<svelte:head>
  <title>diary.computer</title>
</svelte:head>

<div
  class="app"
  class:left-menu-open={uiStore.leftMenuOpen}
  class:left-menu-open-mobile={leftMenuOpenMobile}
  class:left-menu-dragging={isDragging}
  style="--app-left-menu-width: {uiStore.leftMenuWidth}px">
  <div class="top-bar">
    <div class="left">
      <div class="left-menu-toggle desktop">
        <Button
          type="navigation"
          onclick={toggleLeftMenu}
          aria-label="Toggle Menu">
          {#if uiStore.leftMenuOpen}
            <PanelLeftClose />
          {:else}
            <PanelLeftOpen />
          {/if}
        </Button>
      </div>

      <div class="left-menu-toggle mobile">
        <Button
          type="navigation"
          onclick={toggleLeftMenuMobile}
          aria-label="Toggle Menu">
          {#if leftMenuOpenMobile}
            <PanelLeftClose />
          {:else}
            <PanelLeftOpen />
          {/if}
        </Button>
      </div>
    </div>

    <div class="right">
      <Button href="/app/entry/today" left>
        <CalendarDays />
        Today
      </Button>
    </div>
  </div>

  <div class="left-menu">
    <div class="items">
      <div class="actions">
        <Button type="navigation" href="/app/" left>
          <House />
          <div class="ellipsis">Home</div>
        </Button>

        <Button type="navigation" href="/app/stats" left>
          <ChartLine />
          <div class="ellipsis">Stats</div>
        </Button>

        <Button type="navigation" href="/app/entries" left>
          <ScrollText />
          <div class="ellipsis">Entries</div>
        </Button>

        <Button type="navigation" href="/app/categories" left>
          <FolderOpen />
          <div class="ellipsis">Categories</div>
        </Button>

        <Button type="navigation" href="/app/tags" left>
          <Tags />
          <div class="ellipsis">Tags</div>
        </Button>
      </div>

      <div class="calendar">
        <Calendar usedata />
      </div>

      <div class="footer">
        {#if userStore.userDetails}
          <div class="user">
            <Button
              type="navigation"
              fullwidth
              onclick={() => (userDetailsModal = true)}
              left>
              <User />
              <div class="ellipsis">
                {userStore.userDetails.name}
              </div>
            </Button>
          </div>
          <div class="settings">
            <Button
              type="navigation"
              onclick={() => {
                settingsModal = true
              }}
              left>
              <Settings />
            </Button>
          </div>
        {/if}
      </div>
    </div>

    <div class="drag-area">
      <button onmousedown={startDrag} aria-label="Drag to resize left menu"
      ></button>
    </div>
  </div>

  <div class="content">
    {@render children()}
  </div>
</div>

{#if userStore.userDetails}
  <Modal bind:open={userDetailsModal}>
    <div class="app-modal user-details">
      <div class="title">
        <User />
        {userStore.userDetails.name}
      </div>

      <div class="muted small">
        {userStore.userDetails.email}
      </div>

      <div class="user-actions">
        <Button href="/app/user/" fullwidth>
          <UserCog />
          Manage account
        </Button>

        <Button type="destructive" onclick={handleLogout} fullwidth>
          <LogOut />
          Log out
        </Button>
      </div>
    </div>
  </Modal>
{/if}

<Modal bind:open={settingsModal}>
  <div class="app-modal settings-modal">
    <div class="title">Settings</div>

    <div class="settings-options">
      <div class="form-field inline">
        <Label>Theme</Label>
        <ThemePicker />
      </div>

      <div class="form-field inline">
        <Label>Accent Colour</Label>
        <ColorPicker bind:value={uiStore.color} />
      </div>
    </div>

    <div class="settings-actions">
      <div class="internal">
        <div class="internal-element">
          <Button href="/" fullwidth>
            <Logo /> Landing Page
          </Button>
        </div>

        <div class="internal-element">
          <Button href="/docs" fullwidth>
            <Book /> Docs
          </Button>
        </div>
      </div>

      <Button
        href="https://github.com/alexampersandria/diary.computer"
        target="_blank"
        fullwidth>
        <Github /> GitHub
      </Button>
    </div>
  </div>
</Modal>

<style lang="scss">
.app {
  height: 100dvh;
  overflow: hidden;
  display: grid;
  grid-template-areas:
    'top-bar top-bar'
    'left-menu content';
  grid-template-rows: var(--app-top-bar-height) calc(
      100dvh - var(--app-top-bar-height)
    );
  grid-template-columns: var(--app-left-menu-minimized-width) 1fr;
  background-color: var(--background-secondary);
  transition:
    grid-template-columns var(--animation-length-s) var(--better-ease-out),
    background-color var(--animation-length-s) var(--better-ease-out),
    border-color var(--animation-length-s) var(--better-ease-out);

  &.left-menu-open {
    grid-template-columns:
      max(
        min(
          calc(var(--app-left-menu-width) - var(--padding-s)),
          var(--app-left-menu-max-width)
        ),
        var(--app-left-menu-min-width)
      )
      1fr;
  }

  .left-menu-toggle {
    &.mobile {
      display: none;
    }
  }

  &.left-menu-dragging {
    transition: none;
    cursor: ew-resize;
    user-select: none;
  }

  .content,
  .left-menu,
  .top-bar {
    transition:
      background-color var(--animation-length-s) var(--better-ease-out),
      border-color var(--animation-length-s) var(--better-ease-out);
  }

  .content {
    position: relative;
    grid-area: content;
    background-color: var(--background-primary);
    border-radius: var(--radius-m);
    border: var(--border-width) solid var(--border-color);
    margin-right: var(--padding-s);
    margin-bottom: var(--padding-s);
    margin-left: var(--padding-s);
    overflow: auto;
  }

  .top-bar {
    grid-area: top-bar;
    display: flex;
    justify-content: space-between;
    padding: 0 var(--padding-s);
    gap: var(--padding-s);

    .left,
    .right {
      display: flex;
      align-items: center;
      gap: calc(var(--padding-xs) - var(--border-width));
    }

    .right {
      flex-shrink: 1;
    }
  }

  .left-menu {
    grid-area: left-menu;
    position: relative;
    display: flex;
    flex-direction: column;
    padding-bottom: var(--padding-m);
    overflow-x: visible;
    width: calc(100% + var(--padding-s));
    height: calc(100% + (var(--padding-s) * 2));

    .ellipsis {
      animation: fadeInEllipsis var(--animation-length-s) var(--better-ease-out)
        forwards;
      animation-delay: var(--animation-length-s);
      animation-fill-mode: backwards;
    }

    .items {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: var(--padding-s);
      padding: var(--padding-s);
      position: relative;
      top: calc(-1 * var(--padding-s));
      height: 100%;
      overflow: hidden;
      width: 100%;

      .actions,
      .footer {
        gap: var(--padding-xxs);
      }

      .actions {
        display: flex;
        flex-direction: column;
        flex-shrink: 1;
        overflow: auto;
        width: calc(100% + var(--padding-s) * 2);
        margin-left: calc(-1 * var(--padding-s));
        margin-top: calc(-1 * var(--padding-s));
        padding: var(--padding-s);
      }

      .footer {
        display: flex;
        flex-shrink: 0;

        .user {
          display: flex;
          flex-grow: 1;
          overflow: hidden;
        }

        .settings {
          display: flex;
          animation: fadeInEllipsis var(--animation-length-s)
            var(--better-ease-out) forwards;
          animation-delay: var(--animation-length-s);
          animation-fill-mode: backwards;
        }
      }

      .calendar {
        margin-top: auto;
        padding: var(--padding-xxs);
        animation: fadeInEllipsis var(--animation-length-s)
          var(--better-ease-out) backwards;
        animation-delay: var(--animation-length-s);
      }
    }

    .drag-area {
      --drag-area-width: calc(var(--padding-s) * 2);
      position: absolute;
      top: 0;
      right: calc(var(--drag-area-width) * -0.5);

      width: var(--drag-area-width);
      height: calc(100% - (var(--padding-s) * 3));
      cursor: ew-resize;

      button {
        all: unset;
        display: block;
        width: 100%;
        height: 100%;
      }
    }
  }

  &:not(.left-menu-open) {
    .left-menu {
      .ellipsis {
        animation: fadeOutEllipsis var(--animation-length-s)
          var(--better-ease-out) forwards;
      }

      .drag-area {
        display: none;
      }

      .footer {
        flex-direction: column-reverse;

        .settings {
          animation: none;
          animation: fadeIn var(--animation-length-s) var(--better-ease-out)
            forwards;
          animation-delay: var(--animation-length-s);
          animation-fill-mode: backwards;
        }

        @keyframes fadeIn {
          0% {
            opacity: 0;
          }
          1% {
            opacity: 0;
          }
          100% {
            opacity: 1;
          }
        }
      }

      .calendar {
        animation: none;
        display: none;
      }
    }
  }

  @keyframes fadeOutEllipsis {
    0% {
      opacity: 1;
      display: block;
    }
    99% {
      opacity: 0;
      display: block;
    }
    100% {
      opacity: 0;
      display: none;
    }
  }

  @keyframes fadeInEllipsis {
    0% {
      opacity: 0;
      display: none;
    }
    1% {
      opacity: 0;
      display: block;
    }
    100% {
      opacity: 1;
      display: block;
    }
  }
}

@media (max-width: 768px) {
  .app {
    display: flex;
    flex-direction: column;
    position: relative;

    .left-menu-toggle {
      &.mobile {
        display: block;
      }

      &.desktop {
        display: none;
      }
    }

    .top-bar {
      height: var(--app-top-bar-height);
      flex-shrink: 0;
    }

    .content {
      margin: 0;
      border-radius: 0;
      border-left-width: 0;
      border-right-width: 0;
      border-bottom-width: 0;
      flex-grow: 1;
    }

    .left-menu {
      background-color: var(--background-secondary);
      position: absolute;
      top: calc(var(--app-top-bar-height) - var(--border-width));
      left: 0;
      width: 100%;
      height: calc(100dvh - var(--app-top-bar-height) + var(--border-width));
      z-index: 10;
      padding-bottom: 0;

      transform: translateX(-2rem);
      filter: blur(4px);
      opacity: 0;
      pointer-events: none;
      transition:
        transform var(--animation-length-s) var(--better-ease-out),
        filter var(--animation-length-s) var(--better-ease-out),
        opacity var(--animation-length-s) var(--better-ease-out);

      .items {
        padding: var(--padding-s);
        top: 0;

        .actions,
        .footer {
          .ellipsis {
            animation: none;
          }
        }

        .footer {
          flex-direction: row;

          .settings {
            animation: none !important;
          }
        }
      }
    }

    &.left-menu-open-mobile {
      .left-menu {
        transform: translateX(0);
        filter: blur(0);
        opacity: 1;
        pointer-events: auto;
      }
    }
  }
}

.app-modal {
  display: flex;
  flex-direction: column;

  .title {
    font-size: var(--font-size-l);
    font-weight: 600;
  }

  .user-actions {
    padding-top: var(--padding-m);
  }

  .user-actions,
  .settings-actions {
    display: flex;
    flex-direction: column;
    gap: var(--padding-xs);
  }

  .internal {
    display: flex;
    gap: var(--padding-xs);

    .internal-element {
      flex: 1;
    }
  }

  &.settings-modal {
    .settings-options {
      display: flex;
      flex-direction: column;
      gap: var(--padding-s);
      padding-block: var(--padding-m);
    }

    .settings-actions {
      padding-top: var(--padding-s);
    }
  }
}
</style>
