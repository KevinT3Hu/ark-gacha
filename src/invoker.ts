import { invoke } from "@tauri-apps/api";
import { Gacha, PoolStatistics, TotalStatistics, UserCredential } from "./types";

class CommandInvoker {

    public static async saveCredential(i_credential:UserCredential): Promise<void> {
        return invoke("save_auth_credentials",{
            credential: i_credential
        })
    }

    public static async getCredential(): Promise<UserCredential> {
        return invoke("get_auth_credentials")
    }

    public static async login(credential:UserCredential): Promise<void> {
        return invoke("login", {
            credential: credential
        })
    }

    public static async getAllGacha(): Promise<Gacha[]> {
        return invoke("get_all_gacha")
    }

    public static async calculateStatistics(gacha:Gacha[], pool?:string): Promise<TotalStatistics|PoolStatistics> {
        return invoke("calculate_statistics", {
            gacha: gacha,
            pool: pool
        })
    }
}

export default CommandInvoker;