import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import styles from "./ModalWindow.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";
import { format_price } from "../helpers";

const OpenCreditModalContent: FC = () => {
  const [form_data, set_form_data] = useState({
    amount: "",
  });
  const [button_disabled, set_button_disabled] = useState(true);
  const card_numbers = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  ).filter((card_number) => card_number !== "01");

  function handle_change(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e.target.value;
    let only_digit = value.replace(/[^\d]/g, "");

    set_form_data({
      amount: only_digit,
    });
  }

  function handle_submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    card_numbers.forEach((card_number) =>
      console.log(`${form_data.amount} coins was credit to ${card_number}`)
    );
  }

  useEffect(() => {
    if (form_data.amount !== "") {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

  return (
    <>
      <h2 className={styles.h2}>Open Credit</h2>
      <form
        onSubmit={handle_submit}
        className={styles.submit_form}
      >
        <p
          className={styles.info_message}
          style={{
            marginBottom: "2rem",
          }}
        >
          How much money do you want to credit?
        </p>
        <input
          type="text"
          name="password"
          autoFocus
          value={
            form_data.amount
              ? format_price(Number.parseFloat(form_data.amount))
              : ""
          }
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

export default OpenCreditModalContent;
