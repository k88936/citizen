# Agent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | An integer value that is the unique agent identificator. | [optional]
**name** | Option<**String**> | The public agent name. Cloud agent names are typically formed by appending the unique instance name to the parent cloud profile name. | [optional]
**type_id** | Option<**i32**> | Returns the unique ID of the agent category. Local standalone have unique type IDs whereas cloud agents spawned from the same cloud image have the same type ID. | [optional]
**connected** | Option<**bool**> | Returns *true* if the agent is connected to the TeamCity server; otherwise, *false*. Connected agents are those that can communicate with the TeamCity server. | [optional]
**enabled** | Option<**bool**> | Returns *true* if the agent is enabled; otherwise, *false*. Enabled agents are those that are ready to process new bulds.  Send `PUT` requests to the [`/app/rest/agents/{agentLocator}/enabledInfo`](https://www.jetbrains.com/help/teamcity/rest/agentapi.html#setEnabledInfo) endpoint to manually enable or disable agents. | [optional]
**authorized** | Option<**bool**> | Returns *true* if the agent is authorized on the server; otherwise, *false*. Authorized agents are those that are granted permission to execute building tasks on this TeamCity server.  Send `PUT` requests to the [`/app/rest/agents/{agentLocator}/authorizedInfo`](https://www.jetbrains.com/help/teamcity/rest/agentapi.html#setAuthorizedInfo) endpoint to manually authorize or unauthorize agents. | [optional]
**uptodate** | Option<**bool**> | Returns *true* if both `outdated` and `pluginsOutdated` properties return *false*; *false* if any of these properties returns *true*. | [optional]
**outdated** | Option<**bool**> | Returns *true* for outdated agents that require an update; *false* for agents that are up to date. Agents typically update their software automatically and do not require any actions from your side. | [optional]
**plugins_outdated** | Option<**bool**> | Returns *false* if all plugins installed on this agent are of the latest version; *true* if some of plugins are outdated. Plugins installed from the JetBrains Marketplace are typically updated automatically. | [optional]
**java_outdated** | Option<**bool**> | Returns *true* the Java version installed on the agent machine is outdated; otherwise, *false*. | [optional]
**ip** | Option<**String**> | The IP address of the agent machine. | [optional]
**protocol** | Option<**Protocol**> | Returns the agent-server communication protocol type:  * *unidirectional* — the default one-way communication protocol. Unidirectional agents establish an HTTP(S) connection to the TeamCity server, and periodically poll it for server commands. * *bidirectional* — [no longer supported](https://www.jetbrains.com/help/teamcity/upgrade-notes.html#Canceled+bidirectional+agent-server+communication+protocol) in TeamCity 2021.2 and newer. (enum: unidirectional, bidirectional) | [optional]
**version** | Option<**String**> | The actual version of the agent software or 'unknown' if the agent is disconnected. If this version differs from the version expected by the server (the `currentAgentVersion` property), the agent is considered outdated. | [optional]
**current_agent_version** | Option<**String**> | The version of the agent software that is expected by the current TeamCity server. Matches the build number of this server (can be obtained from the `/app/rest/server/build` endpoint). | [optional]
**last_activity_time** | Option<**String**> | The timestamp of the last comminication between this agent and the TeamCity server.  To get the last time this agent performed an activity different from periodic server polling (for example, ran a build), use the `idleSinceTime` property instead. | [optional]
**idle_since_time** | Option<**String**> | The timestamp of the last meaningful agent activity (for example, running a build). For agents that never ran a build after they were authorized by the server, this timestamp is typically equal to the `registrationTimestamp` property value. | [optional]
**disconnection_comment** | Option<**String**> | The summary that explains why this agent was disconnected. Returns `null` for connected agents. | [optional]
**registration_timestamp** | Option<**String**> | The timestamp of the first successful agent authorization on this server. | [optional]
**host** | Option<**String**> | The name of the agent machine host. | [optional]
**cpu_rank** | Option<**i32**> | An integer value that is the [benchmarking score](https://www.jetbrains.com/help/teamcity/viewing-build-agent-details.html#Agent+Summary) of the agent machine's CPU. | [optional]
**port** | Option<**i32**> | The port used by the TeamCity server to connect to the agent. | [optional]
**href** | Option<**String**> | Returns the shortened (without the server URL) link to the current agent. To obtain the full URL, use the *webUrl* property instead. | [optional]
**web_url** | Option<**String**> | Returns the full web link to access this agent. To get a shortened link without the server URL, read the *href* property instead. | [optional]
**build** | Option<[**models::Build**](Build.md)> |  | [optional]
**links** | Option<[**models::Links**](Links.md)> |  | [optional]
**enabled_info** | Option<[**models::EnabledInfo**](EnabledInfo.md)> |  | [optional]
**authorized_info** | Option<[**models::AuthorizedInfo**](AuthorizedInfo.md)> |  | [optional]
**properties** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**cloud_instance** | Option<[**models::CloudInstance**](CloudInstance.md)> |  | [optional]
**cloud_image** | Option<[**models::CloudImage**](CloudImage.md)> |  | [optional]
**environment** | Option<[**models::Environment**](Environment.md)> |  | [optional]
**pool** | Option<[**models::AgentPool**](AgentPool.md)> |  | [optional]
**compatibility_policy** | Option<[**models::CompatibilityPolicy**](CompatibilityPolicy.md)> |  | [optional]
**compatible_build_types** | Option<[**models::BuildTypes**](BuildTypes.md)> |  | [optional]
**incompatible_build_types** | Option<[**models::Compatibilities**](Compatibilities.md)> |  | [optional]
**builds** | Option<[**models::Builds**](Builds.md)> |  | [optional]
**removed** | Option<**bool**> | Returns *true* if the agent was removed after a long period of inactivity; otherwise, *false*. | [optional]
**locator** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


