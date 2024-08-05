export type TUser = {
  id?: { tb: "account"; id: { String: string } };
  name: string;
  account_type: "Blum" | "Dogs";
  init_data: string;
  access_token: string;
  refresh_token: string;
  data?: {
    total_balance?: number;
    play_passes?: number;
  };
  status: boolean;
};
