<script lang="ts">
import { ref } from 'vue';
import { bootcamp_chat_backend } from '../../declarations/bootcamp_chat_backend';


export default {
  data() {
    return {
      newNote: ""
      notes: [] as string[]
    }
  },
  methods: {
    async didajNotatke(){
      await bootcamp_chat_backend.add_note(this.newNote)
      await this.pobierz_notatki()
    },
    async pobierz_notatki(){
      this.notes = await bootcamp_chat_backend.get_notes()
    }
  },
  mounted() {
    this.pobierz_notatki()
  },
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <div>
        {{newNote}}
    </div>
    <div>
        <textarea v-model="newNote"></textarea><button @click="dodajNotatke">dodaj notatke</button>
    </div>
  </main>
</template>
