// store.js
import { writable } from 'svelte/store';
import { type TUser } from '../../types';

let id = 0;
function createUsersStore() {
    const { subscribe, set, update } = writable<TUser[]>([]);
  
    return {
      subscribe,
      set,
      update,
      addUser: (user: TUser) => update((users) => {
        console.log('Adding user:', user); // Custom logic before update
        return [...users, {...user, id: ++id}];
      }),
      updateUser: (user: TUser) => update((users) => {
        console.log('Updating user with id:', user.id); // Custom logic before update
        return users.map(u => u.id === user.id ? user : u);
      }),
      removeUser: (id: number) => update(users => {
        console.log('Removing user with id:', id); // Custom logic before update
        return users.filter(u => u.id !== id);
      })
    };
  }
  
  export const users = createUsersStore();