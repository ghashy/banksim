import "./App.scss";
import { Routes, BrowserRouter, Route } from "react-router-dom";
import MainLayout from "./Components/MainLayout";
import MainPage from "./Pages/MainPage";
import TransactionsPage from "./Pages/TransactionsPage";
import LogsPage from "./Pages/LogsPage";
import AboutPage from "./Pages/AboutPage";
import NotFoundPage from "./Pages/NotFoundPage";
import { useEffect } from "react";
import { API_URL } from "./config";
import useAxios from "./hooks/useAxios";
import { useDispatch } from "react-redux";
import { set_acccount_list } from "./state/account_list_slice";
import {
  set_store_balance,
  set_store_card,
  set_store_emission,
} from "./state/store_info_slice";

const username = "ghashy";
const password = "terminalpassword";
const token = btoa(`${username}:${password}`);
const auth_header = `Basic ${token}`;

type GetEndpoints =
  | "list_accounts"
  | "emission"
  | "store_card"
  | "store_balance";

function App() {
  const { fetch_data: fetch_data } = useAxios();
  const dispatch = useDispatch();

  async function get_data(endpoint: GetEndpoints) {
    const response = await fetch_data({
      method: "GET",
      url: `${API_URL}/system/${endpoint}`,
      headers: {
        Authorization: auth_header,
      },
    });

    if (response?.status === 200) {
      switch (endpoint) {
        case "list_accounts":
          dispatch(set_acccount_list(response.data.accounts));
          break;
        case "emission":
          dispatch(set_store_emission(response.data));
          break;
        case "store_card":
          dispatch(set_store_card(response.data));
          break;
        case "store_balance":
          dispatch(set_store_balance(response.data));
          break;
      }
    }
  }

  useEffect(() => {
    // Get account list
    get_data("list_accounts");

    // Get store info
    async function get_store_info() {
      await get_data("emission");
      await get_data("store_balance");
      await get_data("store_card");
    }
    get_store_info();
  }, []);

  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<MainLayout />}
        >
          <Route
            index
            element={<MainPage />}
          />
          <Route
            path="transactions"
            element={<TransactionsPage />}
          />
          <Route
            path="logs"
            element={<LogsPage />}
          />
          <Route
            path="about"
            element={<AboutPage />}
          />
          <Route
            path="*"
            element={<NotFoundPage />}
          />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
