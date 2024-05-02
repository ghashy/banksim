import { useDispatch, useSelector } from "react-redux";
import AccountTable from "../Components/AccountTable";
import ActionButton from "../UI/ActionButton";
import styles from "./MainPage.module.scss";
import { FC, useState } from "react";
import { RootState } from "../state/store";
import ModalWindow from "../Components/ModalWindow";
import { ActionKind } from "../types";
import { reset_checked_itmes } from "../state/checked_items_slice";

interface ModalVisible {
  kind: ActionKind;
  visible: boolean;
}

const MainPage: FC = () => {
  const [modal_visible, set_modal_visible] = useState<ModalVisible>({
    kind: "",
    visible: false,
  });
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );
  const dispatch = useDispatch();

  function handle_show_modal(kind: ActionKind) {
    set_modal_visible({
      kind: kind,
      visible: true,
    });
  }

  function handle_hide_modal() {
    set_modal_visible({
      kind: "",
      visible: false,
    });
  }

  return (
    <section className={styles.main_page}>
      {modal_visible.visible && (
        <ModalWindow
          kind={modal_visible.kind}
          hide_window={handle_hide_modal}
        />
      )}
      <div className={styles.content}>
        <div className={styles.header_container}>
          <div className={styles.heading}>
            <h2 className={styles.h2}>Accounts</h2>
            {checked_items.length !== 0 && (
              <div
                className={styles.unselect_all}
                onClick={() => dispatch(reset_checked_itmes())}
              >
                (unselect all)
              </div>
            )}
          </div>
          <div className={styles.action_buttons}>
            {checked_items.length === 0 ? (
              <>
                <ActionButton
                  kind="new_transaction"
                  show_modal={handle_show_modal}
                />
                <ActionButton
                  kind="new_account"
                  show_modal={handle_show_modal}
                />
              </>
            ) : (
              <>
                <ActionButton
                  kind="open_credit"
                  show_modal={handle_show_modal}
                />
                <ActionButton
                  kind="delete_account"
                  show_modal={handle_show_modal}
                />
              </>
            )}
          </div>
        </div>
        <AccountTable />
      </div>
    </section>
  );
};

export default MainPage;
