0x802ff7c0:
nop ; Get rid of ClearArena

0x800063ec:
bl init

0x80006458:
bl game_loop
bl 0x80022e74 ; fapGm_Execute__Fv

0x80313b10:
b set_control_stuff