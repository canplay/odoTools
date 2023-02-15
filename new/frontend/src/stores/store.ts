import { defineStore } from 'pinia';

export const useStore = defineStore('global', {
  state: () => ({
    backend: '',
    signin: false,
  }),
});
