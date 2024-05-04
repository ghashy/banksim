import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import styles from "./ModalWindow.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";
import { format_price } from "../helpers";
import useAxios from "../hooks/useAxios";
import { API_URL, AUTH_HEADER } from "../config";

interface OpenCreditModalContentProps {
  hide_window: () => void;
}

interface FormData {
  amount: number | null;
}

const OpenCreditModalContent: FC<OpenCreditModalContentProps> = ({
  hide_window,
}) => {
  const [form_data, set_form_data] = useState<FormData>({
    amount: null,
  });
  const [button_disabled, set_button_disabled] = useState(true);
  const card_numbers = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  ).filter((card_number) => card_number !== "01");
  const { fetch_data: open_credit } = useAxios();

  function handle_change(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e.target.value;
    let only_digit = value.replace(/[^\d]/g, "");

    set_form_data({
      amount: parseInt(only_digit),
    });
  }

  function handle_submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    card_numbers.forEach(async (card_number) => {
      const data = JSON.stringify({
        card_number: card_number,
        amount: form_data.amount,
      });

      await open_credit({
        method: "POST",
        url: `${API_URL}/system/credit`,
        headers: {
          Authorization: AUTH_HEADER,
          "Content-Type": "application/json",
        },
        data: data,
      });

      //TODO Add error handling
    });

    hide_window();
  }

  useEffect(() => {
    if (form_data.amount) {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

  return (
    <>
      <h2 className={styles.h2}>
        Open {card_numbers.length === 1 ? "Credit" : "Credits"}
      </h2>
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
          value={form_data.amount ? format_price(form_data.amount) : ""}
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
