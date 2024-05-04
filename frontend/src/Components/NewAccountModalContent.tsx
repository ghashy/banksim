import { generateUsername } from "unique-username-generator";
import { API_URL, AUTH_HEADER } from "../config";
import useAxios from "../hooks/useAxios";
import styles from "./ModalWindow.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";

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

    const data = JSON.stringify(form_data);

    const response = await add_account({
      method: "POST",
      url: `${API_URL}/system/account`,
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

  useEffect(() => {
    if (form_data.password !== "") {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

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
          Submit
        </button>
      </form>
    </>
  );
};

export default NewAccountModalContent;
