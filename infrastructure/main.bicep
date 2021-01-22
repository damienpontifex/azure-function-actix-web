param location string = 'australiaeast'
param environmentName string {
  allowed: [
    'dev'
    'prod'
  ]
  default: 'dev'
}
param appName string = 'rustfunction'

var commonTags = {
  Environment: environmentName
  ApplicationName: appName
}

resource rg 'Microsoft.Resources/resourceGroups@2020-06-01' = {
  name: appName
  location: location
  tags: commonTags
}

module app './function.bicep' = {
  name: 'functionApp'
  scope: resourceGroup(rg.name)
  params: {
    appName: appName
    commonTags: commonTags
  }
}

output functionAppName string = app.outputs.function.name
output functionAppUrl string = app.outputs.function.hostname