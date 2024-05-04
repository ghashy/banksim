import styles from "./ModalWindow.module.scss";
import { generateUsername } from "unique-username-generator";
import { API_URL, AUTH_HEADER, IS_SECURE } from "../config";
import useAxios from "../hooks/useAxios";
import { FC, FormEvent, useEffect, useState } from "react";
import ErrorModalContent from "./ErrorModalContent";
import { handle_retry } from "../helpers";

interface NewAccountModalContentProps {
  hide_window: () => void;
}

const NewAccountModalContent: FC<NewAccountModalContentProps> = ({
  hide_window,
}) => {
  const [form_data, set_form_data] = useState({
    username: generateUsername(),
    password: "",
  });
  const [button_disabled, set_button_disabled] = useState(true);
  const [fetching, set_fetching] = useState(false);
  const {
    fetch_data: add_account,
    error_data: error_data,
    set_error_data: set_error_data,
    response_status: error_response_status,
    set_response_status: set_error_response_status,
  } = useAxios();

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

    set_fetching(true);

    const data = JSON.stringify(form_data);

    const response = await add_account({
      method: "POST",
      url: `http${IS_SECURE ? "s" : ""}${API_URL}/system/account`,
      headers: {
        Authorization: AUTH_HEADER,
        "Content-Type": "application/json",
      },
      data: data,
    });

    if (response?.status === 200) {
      hide_window();
    }
  }

  function set_states() {
    set_fetching(false);
    set_error_data("");
    set_error_response_status(0);
  }

  useEffect(() => {
    if (form_data.password !== "") {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

  if (error_data) {
    return (
      <ErrorModalContent
        error_response_status={error_response_status}
        error_data={error_data}
        handle_retry={() => handle_retry(error_response_status, set_states)}
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
