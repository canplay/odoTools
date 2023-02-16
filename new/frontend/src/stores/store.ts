import { defineStore } from 'pinia';

export const useStore = defineStore('global', {
  state: () => ({
    backend: 'http://bdo.cltserver.xyz:51530',
    user: {
      signin: false,
      username: '',
      password: '',
    },
  }),
});
