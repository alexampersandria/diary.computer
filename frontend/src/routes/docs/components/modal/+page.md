<script>
import Modal from '$lib/components/Modal.svelte';
import Button from '$lib/components/Button.svelte';
import Input from '$lib/components/Input.svelte';
import DocsExample from '$lib/components/utils/DocsExample.svelte';
import { Link } from "lucide-svelte"

let open = $state(false)
</script>

# Modal

<DocsExample>
  <Button onclick={() => {open = true}}>
    <Link />
    Share
  </Button>

  <Modal bind:open>
    <h3>Share link</h3>
    <Input fullwidth value="https://github.com/alexampersandria/diary.computer" />
  </Modal>
</DocsExample>

```svelte
<script>
let open = $state(false)
</script>

<Button
  onclick={() => {
    open = true
  }}>
  Show modal
</Button>

<Modal bind:open>This is a modal</Modal>
```
