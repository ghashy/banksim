import { useState } from "react";
import { MAX_RETRIES, RETRY_DELAY_MS } from "../config";
import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
import { wait } from "../helpers";

const useAxios = () => {
  const [response_status, set_response_status] = useState(0);
  const [error_data, set_error_data] = useState<string>("");

  const fetch_data = async (
    config: AxiosRequestConfig,
    attempts: number = 1
  ) => {
    let response: AxiosResponse | undefined = undefined;
    try {
      switch (config.method) {
        case "GET":
          response = await axios.get(config.url || "", config);
          return response;
        case "POST":
          response = await axios.post(config.url || "", config.data, config);
          return response;
        case "DELETE":
          response = await axios.delete(config.url || "", config);
          return response;
      }
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          switch (error.response.status) {
            case 400:
              console.error("Bad request:", error.response);
              set_response_status(400);
              set_error_data("Bad request");
              break;
            case 401:
              console.error("Unathorized: ", error.response);
              set_response_status(401);
              set_error_data("Unathorized. Please, authorize and try again");
              break;
            case 403:
              console.error("Forbidden:", error.response);
              set_response_status(403);
              set_error_data(
                "Forbidden. You don't have rights to make this request"
              );
              break;
            case 404:
              console.error(
                "Not found, check the request: ",
                error.config?.url,
                error.response
              );
              set_response_status(404);
              set_error_data("Not found");
              break;
            case 500:
              if (attempts < MAX_RETRIES) {
                await wait(RETRY_DELAY_MS);
                fetch_data(config, attempts + 1);
              } else {
                console.error("Server error:", error.response);
                set_response_status(500);
                set_error_data(
                  "Internal server error. Please, try again later"
                );
              }
              break;
            default:
              console.error(
                "API error: ",
                error.response.status,
                error.response.data
              );
              set_response_status(error.response.status);
              set_error_data(`${error.response.statusText}`);
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            fetch_data(config, attempts + 1);
          } else {
            console.error("Server is not responding:", error.message);
            set_error_data(
              "Server isn't responding. Please, reload the page and try again"
            );
          }
        } else {
          console.error("API Error: Reqest setup error:", error.message);
        }
      } else {
        console.error("Non-Axios:", error);
      }
    }
  };

  return {
    error_data,
    set_error_data,
    response_status,
    set_response_status,
    fetch_data,
  };
};

export default useAxios;
