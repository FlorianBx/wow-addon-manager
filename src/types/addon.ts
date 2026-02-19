export interface Addon {
  id: string
  name: string
  description: string
  icon?: string
  repo: string
  version: string
}

export interface InstalledAddon {
  id: string
  version: string
  installedAt: string
}

export type AddonStatus = 'not-installed' | 'installed' | 'update-available'

export interface AddonWithStatus extends Addon {
  status: AddonStatus
  installedVersion?: string
}
