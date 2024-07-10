// src/controllers/userController.ts
import db from '../../db';
import type { TUser } from '../../types';


export const getAllUsers = async (req: Request, res: Response) => {
    try {
        const users = await db.select<TUser[]>('user');
        res.status(200).json(users);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
};

export const getUser = async (req: Request, res: Response) => {
    try {
        const user = await db.select<TUser>(`user:${req.params.id}`);
        if (user) {
            res.status(200).json(user);
        } else {
            res.status(404).json({ message: 'User not found' });
        }
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
};

export const createUser = async (req: Request, res: Response) => {
    try {
        const newUser = req.body as TUser;
        const createdUser = await db.create<TUser>('user', newUser);
        res.status(201).json(createdUser);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
};

export const updateUser = async (id: string, params: TUser) => {
    try {
        const user = await db.set<TUser>(`user:${id}`, params);
  
};

export const deleteUser = async (id: string) => {
       let response = await db.delete(`user:${id}`);
       return response
};
