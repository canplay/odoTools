import { defineStore } from 'pinia';

export const useStore = defineStore('global', {
  state: () => ({
    // backend: '/backend',
    // backend: 'http://localhost:51530',
    backend: 'http://cltgames.ddns.me:51530',
    user: {
      signin: false,
      username: '',
      password: '',
      familyname: '',
      totalPlayTime: 0,
      membershipType: 0,
    },
    update: {
      remote: {} as any,
      progress: 100,
      status: '已是最新',
    },
  }),
});
