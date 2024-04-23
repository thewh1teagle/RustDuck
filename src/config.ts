export interface Domain {
  name: string;
  enable?: boolean;
}

export interface DomainsConfig {
  email: string;
  token: string;
  interval_minutes?: number;
  domains: Domain[];
  enable: boolean;
}
