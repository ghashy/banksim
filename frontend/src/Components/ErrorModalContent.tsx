import styles from "./ModalWindow.module.scss";
import { FC } from "react";

interface ErrorModalContentProps {
  error_response_status?: number;
  error_data: string;
  recursive?: boolean;
  fetching: boolean;
  handle_retry: () => void;
}

const ErrorModalContent: FC<ErrorModalContentProps> = ({
  error_response_status,
  error_data,
  fetching,
  recursive,
  handle_retry,
}) => {
  return (
    <>
      <h2 className={styles.h2}>
        Error {error_response_status ? error_response_status : "no response"}
      </h2>
      <p className={styles.info_message}>
        {fetching ? (
          <span>
            Retrying<span className={styles.dot1}>.</span>
            <span className={styles.dot2}>.</span>
            <span className={styles.dot3}>.</span>
          </span>
        ) : (
          error_data
        )}
      </p>
      <div
        className={`${styles.button} ${styles.retry_button}`}
        onClick={handle_retry}
      >
        {fetching ? (
          <span className={styles.loader_small}></span>
        ) : recursive ? (
          "Retry"
        ) : (
          "Go back"
        )}
      </div>
    </>
  );
};

export default ErrorModalContent;
