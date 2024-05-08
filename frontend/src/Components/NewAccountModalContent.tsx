import styles from "./ModalWindow.module.scss";
import { generateUsername } from "unique-username-generator";
import { API_URL, AUTH_HEADER, MAX_RETRIES, RETRY_DELAY_MS } from "../config";
import useAxios from "../hooks/useAxios";
import { FC, FormEvent, useEffect, useState } from "react";
import ErrorModalContent from "./ErrorModalContent";
import { wait } from "../helpers";
import { FetchError } from "../types";

interface NewAccountModalContentProps {
  hide_window: () => void;
}

interface FormData {
  username: string;
  password: string;
}

const NewAccountModalContent: FC<NewAccountModalContentProps> = ({
  hide_window,
}) => {
  const [form_data, set_form_data] = useState<FormData>({
    username: generateUsername(),
    password: "",
  });
  const [button_disabled, set_button_disabled] = useState(true);
  const [fetching, set_fetching] = useState(false);
  const [fetch_error, set_fetch_error] = useState<FetchError>({
    message: "",
  });
  const { fetch_data: add_account } = useAxios();

  function handle_change(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e.target.value;

    set_form_data((prev) => ({
      ...prev,
      password: value,
    }));
  }

  async function handle_submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    if (fetching) {
      return;
    }

    const data = JSON.stringify(form_data);
    let attempts = 1;

    recursive_call(data, attempts);
  }

  async function recursive_call(data: string, attempts: number) {
    set_fetching(true);
    const response = await add_account({
      method: "POST",
      url: `${API_URL}/system/account`,
      headers: {
        Authorization: AUTH_HEADER,
        "Content-Type": "application/json",
      },
      data: data,
    });

    if (response.ok) {
      hide_window();
    } else if (
      response.err &&
      response.val.recursive &&
      attempts < MAX_RETRIES
    ) {
      attempts++;
      await wait(RETRY_DELAY_MS);
      recursive_call(data, attempts);
    } else {
      set_fetching(false);
      set_fetch_error({
        message: response.val.message,
        err_status: response.val.err_status,
        recursive: response.val.recursive,
      });
    }
  }

  function retry_fetch() {
    if (fetch_error.recursive) {
      recursive_call(JSON.stringify(form_data), 1);
    } else {
      set_fetch_error({
        message: "",
        err_status: undefined,
        recursive: undefined,
      });
      set_form_data((prev) => ({
        ...prev,
        password: "",
      }));
    }
  }

  useEffect(() => {
    if (form_data.password !== "") {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

  if (fetch_error.message) {
    return (
      <ErrorModalContent
        error_response_status={fetch_error.err_status}
        error_data={fetch_error.message}
        recursive={fetch_error.recursive}
        fetching={fetching}
        handle_retry={retry_fetch}
      />
    );
  }

  return (
    <>
      <h2 className={styles.h2}>New Account</h2>
      <form
        onSubmit={handle_submit}
        className={styles.submit_form}
      >
        <label
          htmlFor="password"
          className={styles.label}
        >
          Password
        </label>
        <input
          type="text"
          name="password"
          id="password"
          autoFocus
          className={styles.text_input}
          onChange={handle_change}
        />
        <button
          type="submit"
          className={styles.submit_button}
          disabled={button_disabled}
        >
          {fetching ? <span className={styles.loader_small}></span> : "Submit"}
        </button>
      </form>
    </>
  );
};

export default NewAccountModalContent;
