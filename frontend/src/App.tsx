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
import {
  set_acccount_list,
  set_accounts_error,
  set_accounts_loading,
} from "./state/account_list_slice";
import {
  set_store_balance,
  set_store_balance_error,
  set_store_balance_loading,
  set_store_card,
  set_store_card_error,
  set_store_card_loading,
  set_store_emission,
  set_store_emission_error,
  set_store_emmision_loading,
} from "./state/store_info_slice";

function App() {
  const { fetch_data: fetch_list, error_data: list_error } = useAxios();
  const { fetch_data: fetch_card, error_data: card_error } = useAxios();
  const { fetch_data: fetch_emission, error_data: emission_error } = useAxios();
  const { fetch_data: fetch_balance, error_data: balance_error } = useAxios();
  const dispatch = useDispatch();

  async function get_account_lsit() {
    const response = await fetch_list({
      method: "GET",
      url: `${API_URL}/system/list_accounts`,
    });

    if (response?.status === 200) {
      dispatch(set_acccount_list(response.data.accounts));
      dispatch(set_accounts_loading(false));
    }
  }

  async function get_store_card() {
    const response = await fetch_card({
      method: "GET",
      url: `${API_URL}/system/store_card`,
    });

    if (response?.status === 200) {
      dispatch(set_store_card(response.data));
      dispatch(set_store_card_loading(false));
    }
  }

  async function get_store_balance() {
    const response = await fetch_balance({
      method: "GET",
      url: `${API_URL}/system/store_balance`,
    });

    if (response?.status === 200) {
      dispatch(set_store_balance(response.data));
      dispatch(set_store_balance_loading(false));
    }
  }

  async function get_store_emission() {
    const response = await fetch_emission({
      method: "GET",
      url: `${API_URL}/system/emission`,
    });

    if (response?.status === 200) {
      dispatch(set_store_emission(response.data));
      dispatch(set_store_emmision_loading(false));
    }
  }

  // Set errors
  useEffect(() => {
    if (list_error) {
      dispatch(set_accounts_error(list_error));
      dispatch(set_accounts_loading(false));
    }
    if (card_error) {
      dispatch(set_store_card_error(card_error));
      dispatch(set_store_card_loading(false));
    }
    if (balance_error) {
      dispatch(set_store_balance_error(balance_error));
      dispatch(set_store_balance_loading(false));
    }
    if (emission_error) {
      dispatch(set_store_emission_error(emission_error));
      dispatch(set_store_emmision_loading(false));
    }
  }, [list_error, card_error, emission_error, balance_error]);

  // Get data
  useEffect(() => {
    // Get account list
    get_account_lsit();

    // Get store info
    try {
      Promise.all([
        get_store_balance(),
        get_store_card(),
        get_store_emission(),
      ]);
    } catch (error) {
      console.error(error);
    }
  }, []);

  //WebSocket connections
  useEffect(() => {
    const socket = new WebSocket(
      `ws://localhost:15100/system/subscribe_on_traces`
    );

    socket.onerror = (e: Event) => {
      console.log(e);
    };

    socket.onopen = () => {
      console.log("WebSocket connection opened");
    };

    socket.onmessage = (e: MessageEvent) => {
      console.log(e.data);
    };

    socket.onclose = () => {
      console.log("WebSocket connection closed");
    };

    return () => {
      socket.close();
    };
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
