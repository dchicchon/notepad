import { STORE_NAME } from './keys';
import { Store } from 'tauri-plugin-store-api';

const store = new Store(STORE_NAME)
export const getKeyVal = async (key) => await store.get(key);
export const setKeyVal = async (key, value) => await store.set(key, value);
