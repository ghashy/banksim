import styles from "./ModalWindow.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";

const NewAccountModalContent: FC = () => {
  const [form_data, set_form_data] = useState({
    password: "",
  });
  const [button_disabled, set_button_disabled] = useState(true);

  function handle_change(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e.target.value;

    set_form_data({
      password: value,
    });
  }

  function handle_submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    console.log(form_data);
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
