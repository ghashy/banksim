import { configureStore } from "@reduxjs/toolkit";
import checked_items_reducer from "./checked_items_slice";

const store = configureStore({
  reducer: {
    checked_items: checked_items_reducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export default store;
