import styles from "./ModalWindow.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";
import { IoChevronDownOutline } from "react-icons/io5";
import { format_price } from "../helpers";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { IAccount } from "../types";

const NewTransactionModalContent: FC = () => {
  const [form_data, set_form_data] = useState({
    from: "",
    to: "",
    amount: "",
  });
  const [button_disabled, set_button_disabled] = useState(true);
  const account_list = useSelector<RootState, IAccount[]>(
    (state) => state.account_list.account_list
  );

  function handle_change(
    e: React.ChangeEvent<HTMLSelectElement | HTMLInputElement>
  ) {
    const { name, value } = e.target;
    let only_digit = value.replace(/[^\d]/g, "");

    if (name === "amount") {
      set_form_data((prev) => ({
        ...prev,
        amount: only_digit,
      }));
      return;
    }

    set_form_data((prev) => ({
      ...prev,
      [name]: value,
    }));
  }

  function handle_submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    console.log(form_data);
  }

  useEffect(() => {
    if (Object.values(form_data).every((field) => field !== "")) {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

  return (
    <>
      <h2 className={styles.h2}>New Transaction</h2>
      <form
        onSubmit={handle_submit}
        className={styles.submit_form}
      >
        <div className={styles.select_container}>
          <IoChevronDownOutline
            className={styles.arrow_down}
            style={{
              color: `${form_data.from ? "#000000" : "#808080"}`,
            }}
          />
          <p className={styles.label}>From</p>
          <select
            name="from"
            id="from"
            value={form_data.from}
            onChange={handle_change}
            className={styles.select}
            style={{
              color: `${form_data.from ? "#000000" : "#808080"}`,
            }}
          >
            <option
              value=""
              disabled
            >
              card number
            </option>
            {account_list.map((account, idx) => (
              <option
                value={account.card_number}
                key={idx}
              >
                {account.card_number}
              </option>
            ))}
          </select>
        </div>
        <div className={styles.select_container}>
          <p className={styles.label}>To</p>
          <IoChevronDownOutline
            className={styles.arrow_down}
            style={{
              color: `${form_data.to ? "#000000" : "#808080"}`,
            }}
          />
          <select
            name="to"
            id="to"
            value={form_data.to}
            onChange={handle_change}
            className={styles.select}
            style={{
              color: `${form_data.to ? "#000000" : "#808080"}`,
            }}
          >
            <option
              value=""
              disabled
            >
              card number
            </option>
            {account_list.map((account, idx) => (
              <option
                value={account.card_number}
                key={idx}
              >
                {account.card_number}
              </option>
            ))}
          </select>
        </div>
        <label
          htmlFor="amount"
          className={styles.label}
        >
          Amount
        </label>
        <input
          type="text"
          id="amount"
          name="amount"
          placeholder="10_000"
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

export default NewTransactionModalContent;
