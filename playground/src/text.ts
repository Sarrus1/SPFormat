export const defaultCode = `/*===================================
=             Variables             =
===================================*/

// Testing Variables
float g_fTick[MAXPLAYERS + 1][2];
float g_fServerLoading[2];
float g_fClientsLoading[MAXPLAYERS + 1][2];
char g_szLogFile[PLATFORM_MAX_PATH];

// PR Commands
int g_iPrTarget[MAXPLAYERS + 1];
int g_totalStagesPr[MAXPLAYERS + 1];
int g_totalBonusesPr[MAXPLAYERS + 1];

// Speed Gradient
char g_szSpeedColour[MAXPLAYERS + 1];

// Show Zones
bool g_bShowZones[MAXPLAYERS + 1];

/*----------  Stages  ----------*/

// Which stage is the client in
int g_Stage[MAXZONEGROUPS][MAXPLAYERS + 1];
int g_WrcpStage[MAXPLAYERS + 1];
bool g_bhasStages;

/*----------  Spawn Locations  ----------*/
float g_fSpawnLocation[MAXZONEGROUPS][CPLIMIT][2][3];
float g_fSpawnAngle[MAXZONEGROUPS][CPLIMIT][2][3];
float g_fSpawnVelocity[MAXZONEGROUPS][CPLIMIT][2][3];
bool g_bGotSpawnLocation[MAXZONEGROUPS][CPLIMIT][2];

public void OnPluginStart(){
    return;
}`;
