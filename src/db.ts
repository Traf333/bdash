// src/db.ts
import Surreal from 'surrealdb.js';


export const db = new Surreal(import.meta.env.VITE_SURREALDB_URL);

export const initDB = async () => {
    await db.signin({
        username: import.meta.env.VITE_SURREALDB_USER,
        password: import.meta.env.VITE_SURREALDB_PASS,
    });

    await db.use({namespace: 'bobr', database: 'bdash'});
};

export default db;
