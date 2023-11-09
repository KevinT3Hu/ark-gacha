export interface UserCredential {
    phone: string;
    password: string;
}

export interface Character {
    name: string;
    rarity: number;
    isNew: boolean
}

export interface Gacha {
    timestamp: number;
    pool: string;
    character: Character;
}

export interface TotalStatistics {
    total: number;
    starsCount: number[];
    starsPercentage: number[];
    allPools: string[];
    poolsCount: number[];
    waterPlace: number[];
}

export interface PoolStatistics {
    allPools: string[];
    poolName: string;
    total: number;
    starsCount: number[];
    starsPercentage: number[];
    waterPlace: number;
}