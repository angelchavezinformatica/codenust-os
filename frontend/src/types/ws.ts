export interface InitResponse {
  user: string
}

export interface ResponseInitWS {
  type: 'init'
  path: string
  data: InitResponse
}

export interface ResponseMessageWS {
  type: 'message'
  path: string
  data: string
}

export type ResponseWS = ResponseInitWS | ResponseMessageWS
