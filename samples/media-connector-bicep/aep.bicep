metadata description = 'Asset endpoint profile for media connector'

@description('Specifies the name of the key vault you are using.')
param keyVaultName string

@description('The RTSP endpoint for the media stream.')
param targetAddress string

@description('The name of the custom location you are using.')
param customLocationName string

@description('Specifies the name of the user-assigned managed identity you are using.')
param uamiName string

@secure()
@description('Specifies the base64 value of the username secret that you want to create.')
param secretValueUsername string

@secure()
@description('Specifies the base64 value of the password secret that you want to create.')
param secretValuePassword string

@description('Specifies the name of the asset endpoint resource to create.')
param aepName string = 'contoso-rtsp-aep-1'

@description('The name of the Kubernetes secret to create.')
param secretName string = 'contoso-secret'

@description('Specifies the name of the SPC resource to create.')
param spcName string = 'contoso-spc'

/*****************************************************************************/
/*                          Existing AIO cluster                             */
/*****************************************************************************/
resource customLocation 'Microsoft.ExtendedLocation/customLocations@2021-08-31-preview' existing = {
  name: customLocationName
}

/*****************************************************************************/
/*                          Add AKV secrets                                  */
/*****************************************************************************/
resource kv 'Microsoft.KeyVault/vaults@2023-07-01' existing = {
  name: keyVaultName
}

resource username 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = {
  parent: kv
  name: 'username'
  properties: {
    value: secretValueUsername
  }
}

resource password 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = {
  parent: kv
  name: 'password'
  properties: {
    value: secretValuePassword
  }
}

/*****************************************************************************/
/*                          Update SPC resource                              */
/* - It's not possible to update an SPC resource using bicep, creating a new */
/*   SPC resource instead.                                                   */
/*****************************************************************************/
resource uami 'Microsoft.ManagedIdentity/userAssignedIdentities@2023-07-31-preview' existing = {
  name: uamiName
}

resource spc 'Microsoft.SecretSyncController/azureKeyVaultSecretProviderClasses@2024-08-21-preview' = {
  name: spcName
  extendedLocation: {
    type: 'CustomLocation'
    name: customLocation.id
  }
  location: resourceGroup().location
  properties: {
    clientId: uami.properties.clientId
    keyvaultName: keyVaultName
    objects: 'array:\n    - |\n      objectName: username\n      objectType: secret\n    - |\n      objectName: password\n      objectType: secret\n'
    tenantId: kv.properties.tenantId
  }
}

/*****************************************************************************/
/*                          Add secretSync                                   */
/*****************************************************************************/
resource secretSync 'Microsoft.SecretSyncController/secretSyncs@2024-08-21-preview' = {
  name: secretName
  extendedLocation: {
    type: 'CustomLocation'
    name: customLocation.id
    }
  location: resourceGroup().location
  properties: {
    kubernetesSecretType: 'Opaque'
    objectSecretMapping: [
        {
        sourcePath: 'username'
        targetKey: 'username'
        }
        {
        sourcePath: 'password'
        targetKey: 'password'
        }
      ]
    secretProviderClassName: spcName
    serviceAccountName: 'aio-ssc-sa'
    }
  }

/*****************************************************************************/
/*                          Asset endpoint profile                           */
/*****************************************************************************/
resource assetEndpoint 'Microsoft.DeviceRegistry/assetEndpointProfiles@2024-11-01' = {
  name: aepName
  location: resourceGroup().location
  extendedLocation: {
    type: 'CustomLocation'
    name: customLocation.id
  }
  properties: {
    targetAddress: targetAddress
    endpointProfileType: 'Microsoft.Media'
    additionalConfiguration: '{"@schema":"https://aiobrokers.blob.core.windows.net/aio-media-connector/1.0.0.json"}'
    authentication: {
      method: 'UsernamePassword'
      usernamePasswordCredentials: {
        passwordSecretName: '${secretName}/password'
        usernameSecretName: '${secretName}/username'
        }
    }
  }
}
