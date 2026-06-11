<template>
  <div
    class="min-h-screen bg-slate-900 text-slate-300 flex flex-col items-center justify-center p-6 font-sans"
  >
    <header class="text-center mb-10">
      <h1 class="text-4xl font-extrabold text-emerald-400 mb-2">
        Cliente de Chat Nativo
      </h1>
      <p class="text-slate-400 text-lg">Nuxt 4 + Tauri + Rust + Tailwind CSS</p>
    </header>

    <div class="w-full max-w-md space-y-6">
      <div
        class="bg-slate-800 border border-slate-700 rounded-xl p-6 shadow-lg text-center"
      >
        <h3 class="text-xl font-bold text-white mb-4">
          Prueba 1: Reactividad Local
        </h3>
        <p class="mb-5 text-slate-300">
          Contador:
          <strong class="text-2xl text-amber-300 ml-2">{{ count }}</strong>
        </p>
        <button
          @click="count++"
          class="bg-blue-500 hover:bg-blue-400 text-white font-semibold py-2 px-6 rounded-lg transition-colors duration-200"
        >
          Incrementar Número
        </button>
      </div>

      <div
        class="bg-slate-800 border border-slate-700 rounded-xl p-6 shadow-lg text-center"
      >
        <h3 class="text-xl font-bold text-white mb-4">
          Prueba 2: Conexión IPC
        </h3>
        <button
          @click="pingRust"
          class="bg-purple-600 hover:bg-purple-500 text-white font-semibold py-2 px-6 rounded-lg transition-colors duration-200"
        >
          Hacer Ping al Backend
        </button>
        <p
          v-if="rustResponse"
          class="mt-5 text-emerald-400 font-bold bg-slate-900/50 py-3 px-4 rounded-lg"
        >
          {{ rustResponse }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const count = ref(0);
const rustResponse = ref("");

async function pingRust() {
  try {
    rustResponse.value = await invoke("ping_backend", { name: "Jesús" });
  } catch (error) {
    console.error("Error al conectar con Tauri:", error);
    rustResponse.value = "Error de conexión con el núcleo de Rust.";
  }
}
</script>
