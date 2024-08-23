// store.js
import { createContext, useReducer } from "react";
const initialState = {
  color: "red",
  console: "",
};

const store = createContext(initialState);
const { Provider } = store;

const StateProvider = ({ children }) => {
  const [state, dispatch] = useReducer((state, action) => {
    switch (action.type) {
      case "CHANGE_COLOR":
        return { ...state, color: action.payload };
      case "CONSOLE":
        return { ...state, console: state.console + action.payload + "\n" };
      default:
        throw new Error();
    }
  }, initialState);
  return <Provider value={{ state, dispatch }}>{children}</Provider>;
};
export { store, StateProvider };