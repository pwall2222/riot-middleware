[Setup]
#define MyAppSetupName 'RiotMiddleware'
#define MyAppVersion '0.1.0'
#define MyAppPublisher 'PWall'
#define MyAppCopyright 'PWall'
#define MyAppURL 'https://github.com/pwall2222/riot-middleware'

AppName={#MyAppSetupName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppSetupName} {#MyAppVersion}
AppCopyright={#MyAppCopyright}
VersionInfoVersion={#MyAppVersion}
VersionInfoCompany={#MyAppPublisher}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
OutputBaseFilename={#MyAppSetupName}
DefaultGroupName={#MyAppSetupName}
DefaultDirName={autopf}\{#MyAppSetupName}
; UninstallDisplayIcon=.ico
; SetupIconFile=.ico
SourceDir=target/release
OutputDir=../../out
AllowNoIcons=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

; remove next line if you only deploy 32-bit binaries and dependencies
ArchitecturesInstallIn64BitMode=x64

[Languages]
Name: en; MessagesFile: "compiler:Default.isl"

[Files]
; Source: "ManifestDownloader.exe"; DestDir: "{app}"; DestName: "ManifestDownloader.exe"; Flags: ignoreversion
Source: "riot_middleware.exe"; DestDir: "{app}"; DestName: "MiddlewareServer.exe"; Flags: ignoreversion
Source: "exec.exe"; DestDir: "{app}"; DestName: "RiotMiddleware.exe"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppSetupName}"; Filename: "{app}\RiotMiddleware.exe"
Name: "{group}\{cm:UninstallProgram,{#MyAppSetupName}}"; Filename: "{uninstallexe}"
Name: "{userdesktop}\{#MyAppSetupName}"; Filename: "{app}\RiotMiddleware.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; Flags: unchecked
