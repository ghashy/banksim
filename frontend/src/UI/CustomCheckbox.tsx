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
  name: string;
}

const CustomCheckbox: FC<CustomCheckboxProps> = ({ name }) => {
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );
  const dispatch = useDispatch();

  function handle_change() {
    if (name === "header") {
      if (checked_items.includes("header")) {
        dispatch(reset_checked_itmes());
      } else {
        let all_checked: string[] = ["header"];
        for (let i = 0; i < accounts.length; i++) {
          all_checked.push(`row${i}`);
        }
        dispatch(set_checked_items(all_checked));
      }

      return;
    }

    if (checked_items.includes(name)) {
      dispatch(
        set_checked_items(checked_items.filter((item) => item !== name))
      );
    } else {
      dispatch(set_checked_items([...checked_items, name]));
    }
  }

  return (
    <label
      htmlFor={name}
      className={styles.custom_checkbox}
    >
      <input
        type="checkbox"
        id={name}
        checked={checked_items.includes(name)}
        onChange={handle_change}
      />
      <div>
        <FaCheck className={styles.checkmark} />
      </div>
    </label>
  );
};

export default CustomCheckbox;
