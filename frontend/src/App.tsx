import "./App.scss";
import { Routes, BrowserRouter, Route } from "react-router-dom";
import MainLayout from "./Components/MainLayout";
import MainPage from "./Pages/MainPage";
import TransactionsPage from "./Pages/TransactionsPage";
import LogsPage from "./Pages/LogsPage";
import AboutPage from "./Pages/AboutPage";
import NotFoundPage from "./Pages/NotFoundPage";
import { useEffect } from "react";
import { API_URL, AUTH_HEADER, MAX_RETRIES, RETRY_DELAY_MS } from "./config";
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
import { set_logs } from "./state/logs_slice";
import { set_account_socket_open } from "./state/account_socket_slice";
import { SocketEndpoints } from "./types";
import { wait } from "./helpers";

function App() {
  const { fetch_data: fetch_list } = useAxios();
  const { fetch_data: fetch_card } = useAxios();
  const { fetch_data: fetch_emission } = useAxios();
  const { fetch_data: fetch_balance } = useAxios();
  const { fetch_data: fetch_token } = useAxios();
  const dispatch = useDispatch();

  async function get_account_lsit() {
    dispatch(set_accounts_loading(true));
    let attempts = 1;

    async function recursive_call() {
      const response = await fetch_list({
        method: "GET",
        url: `${API_URL}/system/list_accounts`,
        headers: {
          Authorization: AUTH_HEADER,
        },
      });

      if (response.ok) {
        dispatch(set_accounts_error(""));
        dispatch(set_acccount_list(response.val.data.accounts));
        dispatch(set_accounts_loading(false));
      } else if (
        response.err &&
        response.val.recursive &&
        attempts < MAX_RETRIES
      ) {
        attempts++;
        await wait(RETRY_DELAY_MS);
        recursive_call();
      } else {
        dispatch(set_accounts_loading(false));
        dispatch(set_accounts_error(response.val.message));
      }
    }

    recursive_call();
  }

  async function get_store_card() {
    dispatch(set_store_card_loading(true));
    let attempts = 1;

    async function recursive_call() {
      const response = await fetch_card({
        method: "GET",
        url: `${API_URL}/system/store_card`,
        headers: {
          Authorization: AUTH_HEADER,
        },
      });

      if (response.ok) {
        dispatch(set_store_card_error(""));
        dispatch(set_store_card(response.val.data));
        dispatch(set_store_card_loading(false));
      } else if (
        response.err &&
        response.val.recursive &&
        attempts < MAX_RETRIES
      ) {
        attempts++;
        await wait(RETRY_DELAY_MS);
        recursive_call();
      } else {
        dispatch(set_store_card_loading(false));
        dispatch(set_store_card_error(response.val.message));
      }
    }

    recursive_call();
  }

  async function get_store_balance() {
    dispatch(set_store_balance_loading(false));
    let attempts = 1;

    async function recursive_call() {
      const response = await fetch_balance({
        method: "GET",
        url: `${API_URL}/system/store_balance`,
        headers: {
          Authorization: AUTH_HEADER,
        },
      });

      if (response.ok) {
        dispatch(set_store_balance_error(""));
        dispatch(set_store_balance(response.val.data));
        dispatch(set_store_balance_loading(false));
      } else if (
        response.err &&
        response.val.recursive &&
        attempts < MAX_RETRIES
      ) {
        attempts++;
        await wait(RETRY_DELAY_MS);
        recursive_call();
      } else {
        dispatch(set_store_balance_loading(false));
        dispatch(set_store_balance_error(response.val.message));
      }
    }

    recursive_call();
  }

  async function get_store_emission() {
    dispatch(set_store_emmision_loading(true));
    let attempts = 1;

    async function recursive_call() {
      const response = await fetch_emission({
        method: "GET",
        url: `${API_URL}/system/emission`,
        headers: {
          Authorization: AUTH_HEADER,
        },
      });

      if (response.ok) {
        dispatch(set_store_emission_error(""));
        dispatch(set_store_emission(response.val.data));
        dispatch(set_store_emmision_loading(false));
      } else if (
        response.err &&
        response.val.recursive &&
        attempts < MAX_RETRIES
      ) {
        attempts++;
        await wait(RETRY_DELAY_MS);
        recursive_call();
      } else {
        dispatch(set_store_emmision_loading(false));
        dispatch(set_store_emission_error(response.val.message));
      }
    }

    recursive_call();
  }

  async function connect_to_socket(endpoint: SocketEndpoints) {
    const token = await get_token();

    if (token) {
      const socket_protocol =
        window.location.protocol === "https:" ? "wss" : "ws";
      const socket_url = API_URL.replace(/^https?/, "");
      const socket = new WebSocket(
        `${socket_protocol}${socket_url}/system/${endpoint}/${token}`
      );
      socket.onopen = () => {
        switch (endpoint) {
          case "subscribe_on_accounts":
            dispatch(set_account_socket_open(true));
            get_account_lsit();
            console.log("Account WebSocket connection opened");
            break;
          case "subscribe_on_traces":
            console.log("Logs WebSocket connection opened");
            break;
        }
      };
      socket.onmessage = (e: MessageEvent) => {
        switch (endpoint) {
          case "subscribe_on_accounts":
            get_account_lsit();
            get_store_balance();
            get_store_emission();
            break;
          case "subscribe_on_traces":
            dispatch(set_logs(e.data));
            break;
        }
      };
      socket.onerror = (e: Event) => {
        console.log(e);
      };
      socket.onclose = () => {
        switch (endpoint) {
          case "subscribe_on_accounts":
            dispatch(set_account_socket_open(false));
            dispatch(set_acccount_list([]));
            console.log("Account WebSocket connection closed");
            break;
          case "subscribe_on_traces":
            console.log("Logs WebSocket connection closed");
            break;
        }
      };
    } else {
      switch (endpoint) {
        case "subscribe_on_accounts":
          console.error("Accounts token fetch failed, try again");
          dispatch(set_account_socket_open(false));
          break;
        case "subscribe_on_traces":
          console.error("Logs token fetch failed, try again");
          break;
      }
    }
  }

  async function get_token() {
    const response = await fetch_token({
      method: "GET",
      url: `${API_URL}/system/ws_token`,
      headers: {
        Authorization: AUTH_HEADER,
      },
    });

    if (response.ok) {
      return response.val.data;
    } else {
      return null;
    }
  }

  // Get data
  useEffect(() => {
    // Connect to sockets
    connect_to_socket("subscribe_on_accounts");
    connect_to_socket("subscribe_on_traces");

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

  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<MainLayout />}
        >
          <Route
            index
            element={<MainPage connect_to_socket={connect_to_socket} />}
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
