import styles from "./CustomCheckbox.module.scss";
import { FC } from "react";
import { FaCheck } from "react-icons/fa6";
import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../state/store";
import {
  reset_checked_itmes,
  set_checked_items,
} from "../state/checked_items_slice";
import { accounts } from "../mock_data";

interface CustomCheckboxProps {
  card_number: string;
}

const CustomCheckbox: FC<CustomCheckboxProps> = ({ card_number }) => {
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );
  const dispatch = useDispatch();

  function handle_change() {
    // Handle "check all" case
    if (card_number === "01") {
      if (checked_items.includes("01")) {
        dispatch(reset_checked_itmes());
      } else {
        let all_checked: string[] = ["01"];
        accounts.forEach((account) => all_checked.push(account.card_number));
        dispatch(set_checked_items(all_checked));
      }

      return;
    }

    // Handle "check some" case
    if (checked_items.includes(card_number)) {
      dispatch(
        set_checked_items(
          checked_items.filter((item) => item !== card_number && item !== "01")
        )
      );
    } else {
      checked_items.length === accounts.length - 1
        ? dispatch(set_checked_items([...checked_items, card_number, "01"]))
        : dispatch(set_checked_items([...checked_items, card_number]));
    }
  }

  return (
    <label
      htmlFor={card_number}
      className={styles.custom_checkbox}
    >
      <input
        type="checkbox"
        id={card_number}
        checked={checked_items.includes(card_number)}
        onChange={handle_change}
      />
      <div>
        <FaCheck className={styles.checkmark} />
      </div>
    </label>
  );
};

export default CustomCheckbox;
