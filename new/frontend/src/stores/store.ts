import { defineStore } from 'pinia';

export const useStore = defineStore('global', {
  state: () => ({
    // backend: '/backend',
    backend: 'http://1.13.22.82:51530',
    user: {
      signin: false,
      username: '',
      password: '',
      familyname: '',
      totalPlayTime: 0,
      membershipType: 99,
    },
    update: {
      remote: {} as any,
      progress: 100,
      status: '已是最新',
    },
  }),
});
