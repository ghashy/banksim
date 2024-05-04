import "./App.scss";
import { Routes, BrowserRouter, Route } from "react-router-dom";
import MainLayout from "./Components/MainLayout";
import MainPage from "./Pages/MainPage";
import TransactionsPage from "./Pages/TransactionsPage";
import LogsPage from "./Pages/LogsPage";
import AboutPage from "./Pages/AboutPage";
import NotFoundPage from "./Pages/NotFoundPage";
import { useEffect } from "react";
import { API_URL, AUTH_HEADER } from "./config";
import useAxios from "./hooks/useAxios";
import { useDispatch } from "react-redux";
import {
  set_acccount_list,
  set_accounts_loading,
} from "./state/account_list_slice";
import {
  set_store_balance,
  set_store_balance_loading,
  set_store_card,
  set_store_card_loading,
  set_store_emission,
  set_store_emmision_loading,
} from "./state/store_info_slice";

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
        Authorization: AUTH_HEADER,
      },
    });

    if (response?.status === 200) {
      switch (endpoint) {
        case "list_accounts":
          dispatch(set_acccount_list(response.data.accounts));
          dispatch(set_accounts_loading(false));
          return;
        case "emission":
          dispatch(set_store_emission(response.data));
          dispatch(set_store_emmision_loading(false));
          return;
        case "store_card":
          dispatch(set_store_card(response.data));
          dispatch(set_store_card_loading(false));
          return;
        case "store_balance":
          dispatch(set_store_balance(response.data));
          dispatch(set_store_balance_loading(false));
          return;
      }
    }

    switch (endpoint) {
      case "list_accounts":
        dispatch(set_accounts_loading(false));
        break;
      case "emission":
        dispatch(set_store_emmision_loading(false));
        break;
      case "store_card":
        dispatch(set_store_card_loading(false));
        break;
      case "store_balance":
        dispatch(set_store_balance_loading(false));
        break;
    }
  }

  // Get data
  useEffect(() => {
    // Get account list
    get_data("list_accounts");

    // Get store info
    try {
      Promise.all([
        get_data("emission"),
        get_data("store_balance"),
        get_data("store_card"),
      ]);
    } catch (error) {
      console.error(error);
    }
  }, []);

  //WebSocket connections
  // useEffect(() => {
  //   const socket = new WebSocket(
  //     `ws://localhost:15100/system/subscribe_on_traces`
  //   );

  //   socket.onerror = (e: Event) => {
  //     console.log(e);
  //   };

  //   socket.onopen = () => {
  //     console.log("WebSocket connection opened");
  //   };

  //   socket.onmessage = (e: MessageEvent) => {
  //     console.log(e.data);
  //   };

  //   socket.onclose = () => {
  //     console.log("WebSocket connection closed");
  //   };

  //   return () => {
  //     socket.close();
  //   };
  // }, []);

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
