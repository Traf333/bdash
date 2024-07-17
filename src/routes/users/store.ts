// store.js
import { writable } from "svelte/store";
import { type TUser } from "../../types";

function createUsersStore() {
  const { subscribe, set, update } = writable<TUser[]>([]);

  return {
    subscribe,
    set,
    update,
    addUser: (user: TUser) =>
      update((users) => {
        console.log("Adding user:", user); // Custom logic before update
        return [...users, user];
      }),
    updateUser: (user: TUser) =>
      update((users) => {
        console.log("Updating user with id:", user.id); // Custom logic before update
        return users.map((u) =>
          u.id.id.String === user.id.id.String ? user : u,
        );
      }),
    removeUser: (id: string) =>
      update((users) => {
        console.log("Removing user with id:", id); // Custom logic before update
        return users.filter((u) => u.id.id.String !== id);
      }),
  };
}

export const toUserId = (id: TUser["id"]) => id.id.String;

export const users = createUsersStore();
