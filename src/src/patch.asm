0x802ff7c0:
nop ; Get rid of ClearArena

0x800063ec:
bl init

0x8000645C:
bl game_loop

0x8005b47c:
b init_save_file