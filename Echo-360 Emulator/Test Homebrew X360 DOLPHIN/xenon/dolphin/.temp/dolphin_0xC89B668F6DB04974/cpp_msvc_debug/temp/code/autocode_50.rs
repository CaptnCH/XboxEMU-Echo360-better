#include "C:\Users\xdfah\Downloads\recompiler\Echo-360\Echo-360 0.0.1\../../dev/src/xenon_launcher/xenonCPU.h"

//////////////////////////////////////////////////////
// Block at 8226B9D0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226B9D0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226B9D0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226B9D0);
		  /* 8226B9D0h */ case    0:  		/* rlwinm. R26, R26, 0, 24, 31 */
		/* 8226B9D0h case    0:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R26,regs.R26);
		/* 8226B9D0h case    0:*/		return 0x8226B9D4;
		  /* 8226B9D4h */ case    1:  		/* bc 12, CR0_EQ, 68 */
		/* 8226B9D4h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226BA18;  }
		/* 8226B9D4h case    1:*/		return 0x8226B9D8;
		  /* 8226B9D8h */ case    2:  		/* lbz R11, <#[R1 + 112]> */
		/* 8226B9D8h case    2:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R1 + 0x00000070) );
		/* 8226B9D8h case    2:*/		return 0x8226B9DC;
		  /* 8226B9DCh */ case    3:  		/* addi R10, R28, 800 */
		/* 8226B9DCh case    3:*/		cpu::op::addi<0>(regs,&regs.R10,regs.R28,0x320);
		/* 8226B9DCh case    3:*/		return 0x8226B9E0;
		  /* 8226B9E0h */ case    4:  		/* lis R9, -32254 */
		/* 8226B9E0h case    4:*/		cpu::op::lis<0>(regs,&regs.R9,0xFFFF8202);
		/* 8226B9E0h case    4:*/		return 0x8226B9E4;
		  /* 8226B9E4h */ case    5:  		/* rlwinm R8, R11, 30, 2, 29 */
		/* 8226B9E4h case    5:*/		cpu::op::rlwinm<0,30,2,29>(regs,&regs.R8,regs.R11);
		/* 8226B9E4h case    5:*/		return 0x8226B9E8;
		  /* 8226B9E8h */ case    6:  		/* mr R3, R31 */
		/* 8226B9E8h case    6:*/		regs.R3 = regs.R31;
		/* 8226B9E8h case    6:*/		return 0x8226B9EC;
		  /* 8226B9ECh */ case    7:  		/* addi R4, R9, -30860 */
		/* 8226B9ECh case    7:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R9,0xFFFF8774);
		/* 8226B9ECh case    7:*/		return 0x8226B9F0;
		  /* 8226B9F0h */ case    8:  		/* rlwinm R30, R11, 0, 28, 31 */
		/* 8226B9F0h case    8:*/		cpu::op::rlwinm<0,0,28,31>(regs,&regs.R30,regs.R11);
		/* 8226B9F0h case    8:*/		return 0x8226B9F4;
		  /* 8226B9F4h */ case    9:  		/* lwzx R5, <#[R8 + R10]> */
		/* 8226B9F4h case    9:*/		cpu::mem::load32z( regs, &regs.R5, (uint32)(regs.R8 + regs.R10 + 0x00000000) );
		/* 8226B9F4h case    9:*/		return 0x8226B9F8;
		  /* 8226B9F8h */ case   10:  		/* bl -8216 */
		/* 8226B9F8h case   10:*/		regs.LR = 0x8226B9FC; return 0x822699E0;
		/* 8226B9F8h case   10:*/		return 0x8226B9FC;
		  /* 8226B9FCh */ case   11:  		/* cmplwi CR6, R30, 0 */
		/* 8226B9FCh case   11:*/		cpu::op::cmplwi<6>(regs,regs.R30,0x00000000);
		/* 8226B9FCh case   11:*/		return 0x8226BA00;
		  /* 8226BA00h */ case   12:  		/* bc 12, CR6_EQ, 24 */
		/* 8226BA00h case   12:*/		if ( regs.CR[6].eq ) { return 0x8226BA18;  }
		/* 8226BA00h case   12:*/		return 0x8226BA04;
		  /* 8226BA04h */ case   13:  		/* lis R11, -32253 */
		/* 8226BA04h case   13:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8203);
		/* 8226BA04h case   13:*/		return 0x8226BA08;
		  /* 8226BA08h */ case   14:  		/* mr R5, R30 */
		/* 8226BA08h case   14:*/		regs.R5 = regs.R30;
		/* 8226BA08h case   14:*/		return 0x8226BA0C;
		  /* 8226BA0Ch */ case   15:  		/* addi R4, R11, 26668 */
		/* 8226BA0Ch case   15:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x682C);
		/* 8226BA0Ch case   15:*/		return 0x8226BA10;
		  /* 8226BA10h */ case   16:  		/* mr R3, R31 */
		/* 8226BA10h case   16:*/		regs.R3 = regs.R31;
		/* 8226BA10h case   16:*/		return 0x8226BA14;
		  /* 8226BA14h */ case   17:  		/* bl -8244 */
		/* 8226BA14h case   17:*/		regs.LR = 0x8226BA18; return 0x822699E0;
		/* 8226BA14h case   17:*/		return 0x8226BA18;
	}
	return 0x8226BA18;
} // Block from 8226B9D0h-8226BA18h (18 instructions)

//////////////////////////////////////////////////////
// Block at 8226BA18h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BA18( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BA18) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BA18);
		  /* 8226BA18h */ case    0:  		/* lis R11, -32255 */
		/* 8226BA18h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BA18h case    0:*/		return 0x8226BA1C;
		  /* 8226BA1Ch */ case    1:  		/* lis R10, -32255 */
		/* 8226BA1Ch case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8201);
		/* 8226BA1Ch case    1:*/		return 0x8226BA20;
		  /* 8226BA20h */ case    2:  		/* cmplwi CR6, R25, 0 */
		/* 8226BA20h case    2:*/		cpu::op::cmplwi<6>(regs,regs.R25,0x00000000);
		/* 8226BA20h case    2:*/		return 0x8226BA24;
		  /* 8226BA24h */ case    3:  		/* addi R30, R11, 5216 */
		/* 8226BA24h case    3:*/		cpu::op::addi<0>(regs,&regs.R30,regs.R11,0x1460);
		/* 8226BA24h case    3:*/		return 0x8226BA28;
		  /* 8226BA28h */ case    4:  		/* addi R27, R10, 5224 */
		/* 8226BA28h case    4:*/		cpu::op::addi<0>(regs,&regs.R27,regs.R10,0x1468);
		/* 8226BA28h case    4:*/		return 0x8226BA2C;
		  /* 8226BA2Ch */ case    5:  		/* bc 4, CR6_EQ, 132 */
		/* 8226BA2Ch case    5:*/		if ( !regs.CR[6].eq ) { return 0x8226BAB0;  }
		/* 8226BA2Ch case    5:*/		return 0x8226BA30;
		  /* 8226BA30h */ case    6:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BA30h case    6:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BA30h case    6:*/		return 0x8226BA34;
		  /* 8226BA34h */ case    7:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BA34h case    7:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BA34h case    7:*/		return 0x8226BA38;
		  /* 8226BA38h */ case    8:  		/* bc 4, CR0_EQ, 12 */
		/* 8226BA38h case    8:*/		if ( !regs.CR[0].eq ) { return 0x8226BA44;  }
		/* 8226BA38h case    8:*/		return 0x8226BA3C;
		  /* 8226BA3Ch */ case    9:  		/* cmpwi CR6, R23, 0 */
		/* 8226BA3Ch case    9:*/		cpu::op::cmpwi<6>(regs,regs.R23,0x00000000);
		/* 8226BA3Ch case    9:*/		return 0x8226BA40;
		  /* 8226BA40h */ case   10:  		/* bc 4, CR6_EQ, 44 */
		/* 8226BA40h case   10:*/		if ( !regs.CR[6].eq ) { return 0x8226BA6C;  }
		/* 8226BA40h case   10:*/		return 0x8226BA44;
	}
	return 0x8226BA44;
} // Block from 8226BA18h-8226BA44h (11 instructions)

//////////////////////////////////////////////////////
// Block at 8226BA44h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BA44( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BA44) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BA44);
		  /* 8226BA44h */ case    0:  		/* mr R3, R31 */
		/* 8226BA44h case    0:*/		regs.R3 = regs.R31;
		/* 8226BA44h case    0:*/		return 0x8226BA48;
		  /* 8226BA48h */ case    1:  		/* bl -3080 */
		/* 8226BA48h case    1:*/		regs.LR = 0x8226BA4C; return 0x8226AE40;
		/* 8226BA48h case    1:*/		return 0x8226BA4C;
		  /* 8226BA4Ch */ case    2:  		/* cmpwi CR6, R23, 0 */
		/* 8226BA4Ch case    2:*/		cpu::op::cmpwi<6>(regs,regs.R23,0x00000000);
		/* 8226BA4Ch case    2:*/		return 0x8226BA50;
		  /* 8226BA50h */ case    3:  		/* mr R5, R27 */
		/* 8226BA50h case    3:*/		regs.R5 = regs.R27;
		/* 8226BA50h case    3:*/		return 0x8226BA54;
		  /* 8226BA54h */ case    4:  		/* bc 12, CR6_EQ, 8 */
		/* 8226BA54h case    4:*/		if ( regs.CR[6].eq ) { return 0x8226BA5C;  }
		/* 8226BA54h case    4:*/		return 0x8226BA58;
		  /* 8226BA58h */ case    5:  		/* mr R5, R30 */
		/* 8226BA58h case    5:*/		regs.R5 = regs.R30;
		/* 8226BA58h case    5:*/		return 0x8226BA5C;
	}
	return 0x8226BA5C;
} // Block from 8226BA44h-8226BA5Ch (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226BA5Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BA5C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BA5C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BA5C);
		  /* 8226BA5Ch */ case    0:  		/* lis R11, -32251 */
		/* 8226BA5Ch case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BA5Ch case    0:*/		return 0x8226BA60;
		  /* 8226BA60h */ case    1:  		/* mr R3, R31 */
		/* 8226BA60h case    1:*/		regs.R3 = regs.R31;
		/* 8226BA60h case    1:*/		return 0x8226BA64;
		  /* 8226BA64h */ case    2:  		/* addi R4, R11, -16084 */
		/* 8226BA64h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC12C);
		/* 8226BA64h case    2:*/		return 0x8226BA68;
		  /* 8226BA68h */ case    3:  		/* bl -8328 */
		/* 8226BA68h case    3:*/		regs.LR = 0x8226BA6C; return 0x822699E0;
		/* 8226BA68h case    3:*/		return 0x8226BA6C;
	}
	return 0x8226BA6C;
} // Block from 8226BA5Ch-8226BA6Ch (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226BA6Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BA6C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BA6C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BA6C);
		  /* 8226BA6Ch */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BA6Ch case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BA6Ch case    0:*/		return 0x8226BA70;
		  /* 8226BA70h */ case    1:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BA70h case    1:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BA70h case    1:*/		return 0x8226BA74;
		  /* 8226BA74h */ case    2:  		/* bc 4, CR0_EQ, 16 */
		/* 8226BA74h case    2:*/		if ( !regs.CR[0].eq ) { return 0x8226BA84;  }
		/* 8226BA74h case    2:*/		return 0x8226BA78;
		  /* 8226BA78h */ case    3:  		/* lwz R11, <#[R29 + 4]> */
		/* 8226BA78h case    3:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BA78h case    3:*/		return 0x8226BA7C;
		  /* 8226BA7Ch */ case    4:  		/* rlwinm. R11, R11, 0, 16, 16 */
		/* 8226BA7Ch case    4:*/		cpu::op::rlwinm<1,0,16,16>(regs,&regs.R11,regs.R11);
		/* 8226BA7Ch case    4:*/		return 0x8226BA80;
		  /* 8226BA80h */ case    5:  		/* bc 12, CR0_EQ, 48 */
		/* 8226BA80h case    5:*/		if ( regs.CR[0].eq ) { return 0x8226BAB0;  }
		/* 8226BA80h case    5:*/		return 0x8226BA84;
	}
	return 0x8226BA84;
} // Block from 8226BA6Ch-8226BA84h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226BA84h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BA84( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BA84) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BA84);
		  /* 8226BA84h */ case    0:  		/* mr R3, R31 */
		/* 8226BA84h case    0:*/		regs.R3 = regs.R31;
		/* 8226BA84h case    0:*/		return 0x8226BA88;
		  /* 8226BA88h */ case    1:  		/* bl -3144 */
		/* 8226BA88h case    1:*/		regs.LR = 0x8226BA8C; return 0x8226AE40;
		/* 8226BA88h case    1:*/		return 0x8226BA8C;
		  /* 8226BA8Ch */ case    2:  		/* lwz R11, <#[R29 + 4]> */
		/* 8226BA8Ch case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BA8Ch case    2:*/		return 0x8226BA90;
		  /* 8226BA90h */ case    3:  		/* mr R5, R27 */
		/* 8226BA90h case    3:*/		regs.R5 = regs.R27;
		/* 8226BA90h case    3:*/		return 0x8226BA94;
		  /* 8226BA94h */ case    4:  		/* rlwinm. R11, R11, 0, 16, 16 */
		/* 8226BA94h case    4:*/		cpu::op::rlwinm<1,0,16,16>(regs,&regs.R11,regs.R11);
		/* 8226BA94h case    4:*/		return 0x8226BA98;
		  /* 8226BA98h */ case    5:  		/* bc 4, CR0_EQ, 8 */
		/* 8226BA98h case    5:*/		if ( !regs.CR[0].eq ) { return 0x8226BAA0;  }
		/* 8226BA98h case    5:*/		return 0x8226BA9C;
		  /* 8226BA9Ch */ case    6:  		/* mr R5, R30 */
		/* 8226BA9Ch case    6:*/		regs.R5 = regs.R30;
		/* 8226BA9Ch case    6:*/		return 0x8226BAA0;
	}
	return 0x8226BAA0;
} // Block from 8226BA84h-8226BAA0h (7 instructions)

//////////////////////////////////////////////////////
// Block at 8226BAA0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BAA0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BAA0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BAA0);
		  /* 8226BAA0h */ case    0:  		/* lis R11, -32251 */
		/* 8226BAA0h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BAA0h case    0:*/		return 0x8226BAA4;
		  /* 8226BAA4h */ case    1:  		/* mr R3, R31 */
		/* 8226BAA4h case    1:*/		regs.R3 = regs.R31;
		/* 8226BAA4h case    1:*/		return 0x8226BAA8;
		  /* 8226BAA8h */ case    2:  		/* addi R4, R11, -16100 */
		/* 8226BAA8h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC11C);
		/* 8226BAA8h case    2:*/		return 0x8226BAAC;
		  /* 8226BAACh */ case    3:  		/* bl -8396 */
		/* 8226BAACh case    3:*/		regs.LR = 0x8226BAB0; return 0x822699E0;
		/* 8226BAACh case    3:*/		return 0x8226BAB0;
	}
	return 0x8226BAB0;
} // Block from 8226BAA0h-8226BAB0h (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226BAB0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BAB0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BAB0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BAB0);
		  /* 8226BAB0h */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BAB0h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BAB0h case    0:*/		return 0x8226BAB4;
		  /* 8226BAB4h */ case    1:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BAB4h case    1:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BAB4h case    1:*/		return 0x8226BAB8;
		  /* 8226BAB8h */ case    2:  		/* bc 4, CR0_EQ, 16 */
		/* 8226BAB8h case    2:*/		if ( !regs.CR[0].eq ) { return 0x8226BAC8;  }
		/* 8226BAB8h case    2:*/		return 0x8226BABC;
		  /* 8226BABCh */ case    3:  		/* lwz R11, <#[R29 + 4]> */
		/* 8226BABCh case    3:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BABCh case    3:*/		return 0x8226BAC0;
		  /* 8226BAC0h */ case    4:  		/* rlwinm. R11, R11, 0, 2, 7 */
		/* 8226BAC0h case    4:*/		cpu::op::rlwinm<1,0,2,7>(regs,&regs.R11,regs.R11);
		/* 8226BAC0h case    4:*/		return 0x8226BAC4;
		  /* 8226BAC4h */ case    5:  		/* bc 12, CR0_EQ, 40 */
		/* 8226BAC4h case    5:*/		if ( regs.CR[0].eq ) { return 0x8226BAEC;  }
		/* 8226BAC4h case    5:*/		return 0x8226BAC8;
	}
	return 0x8226BAC8;
} // Block from 8226BAB0h-8226BAC8h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226BAC8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BAC8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BAC8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BAC8);
		  /* 8226BAC8h */ case    0:  		/* mr R3, R31 */
		/* 8226BAC8h case    0:*/		regs.R3 = regs.R31;
		/* 8226BAC8h case    0:*/		return 0x8226BACC;
		  /* 8226BACCh */ case    1:  		/* bl -3212 */
		/* 8226BACCh case    1:*/		regs.LR = 0x8226BAD0; return 0x8226AE40;
		/* 8226BACCh case    1:*/		return 0x8226BAD0;
		  /* 8226BAD0h */ case    2:  		/* lis R11, -32251 */
		/* 8226BAD0h case    2:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BAD0h case    2:*/		return 0x8226BAD4;
		  /* 8226BAD4h */ case    3:  		/* mr R3, R31 */
		/* 8226BAD4h case    3:*/		regs.R3 = regs.R31;
		/* 8226BAD4h case    3:*/		return 0x8226BAD8;
		  /* 8226BAD8h */ case    4:  		/* addi R4, R11, -16116 */
		/* 8226BAD8h case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC10C);
		/* 8226BAD8h case    4:*/		return 0x8226BADC;
		  /* 8226BADCh */ case    5:  		/* lwz R11, <#[R29 + 4]> */
		/* 8226BADCh case    5:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BADCh case    5:*/		return 0x8226BAE0;
		  /* 8226BAE0h */ case    6:  		/* rlwinm R11, R11, 2, 0, 29 */
		/* 8226BAE0h case    6:*/		cpu::op::rlwinm<0,2,0,29>(regs,&regs.R11,regs.R11);
		/* 8226BAE0h case    6:*/		return 0x8226BAE4;
		  /* 8226BAE4h */ case    7:  		/* srawi R5, R11, 26 */
		/* 8226BAE4h case    7:*/		cpu::op::srawi<0,26>(regs,&regs.R5,regs.R11);
		/* 8226BAE4h case    7:*/		return 0x8226BAE8;
		  /* 8226BAE8h */ case    8:  		/* bl -8456 */
		/* 8226BAE8h case    8:*/		regs.LR = 0x8226BAEC; return 0x822699E0;
		/* 8226BAE8h case    8:*/		return 0x8226BAEC;
	}
	return 0x8226BAEC;
} // Block from 8226BAC8h-8226BAECh (9 instructions)

//////////////////////////////////////////////////////
// Block at 8226BAECh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BAEC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BAEC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BAEC);
		  /* 8226BAECh */ case    0:  		/* cmplwi CR6, R26, 0 */
		/* 8226BAECh case    0:*/		cpu::op::cmplwi<6>(regs,regs.R26,0x00000000);
		/* 8226BAECh case    0:*/		return 0x8226BAF0;
		  /* 8226BAF0h */ case    1:  		/* bc 4, CR6_EQ, 436 */
		/* 8226BAF0h case    1:*/		if ( !regs.CR[6].eq ) { return 0x8226BCA4;  }
		/* 8226BAF0h case    1:*/		return 0x8226BAF4;
		  /* 8226BAF4h */ case    2:  		/* lis R11, -32253 */
		/* 8226BAF4h case    2:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8203);
		/* 8226BAF4h case    2:*/		return 0x8226BAF8;
		  /* 8226BAF8h */ case    3:  		/* mr R3, R31 */
		/* 8226BAF8h case    3:*/		regs.R3 = regs.R31;
		/* 8226BAF8h case    3:*/		return 0x8226BAFC;
		  /* 8226BAFCh */ case    4:  		/* addi R4, R11, 28356 */
		/* 8226BAFCh case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x6EC4);
		/* 8226BAFCh case    4:*/		return 0x8226BB00;
		  /* 8226BB00h */ case    5:  		/* bl -8216 */
		/* 8226BB00h case    5:*/		regs.LR = 0x8226BB04; return 0x82269AE8;
		/* 8226BB00h case    5:*/		return 0x8226BB04;
		  /* 8226BB04h */ case    6:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BB04h case    6:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BB04h case    6:*/		return 0x8226BB08;
		  /* 8226BB08h */ case    7:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BB08h case    7:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BB08h case    7:*/		return 0x8226BB0C;
		  /* 8226BB0Ch */ case    8:  		/* bc 4, CR0_EQ, 16 */
		/* 8226BB0Ch case    8:*/		if ( !regs.CR[0].eq ) { return 0x8226BB1C;  }
		/* 8226BB0Ch case    8:*/		return 0x8226BB10;
		  /* 8226BB10h */ case    9:  		/* lwz R11, <#[R29 + 8]> */
		/* 8226BB10h case    9:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000008) );
		/* 8226BB10h case    9:*/		return 0x8226BB14;
		  /* 8226BB14h */ case   10:  		/* rlwinm. R11, R11, 0, 1, 23 */
		/* 8226BB14h case   10:*/		cpu::op::rlwinm<1,0,1,23>(regs,&regs.R11,regs.R11);
		/* 8226BB14h case   10:*/		return 0x8226BB18;
		  /* 8226BB18h */ case   11:  		/* bc 12, CR0_EQ, 40 */
		/* 8226BB18h case   11:*/		if ( regs.CR[0].eq ) { return 0x8226BB40;  }
		/* 8226BB18h case   11:*/		return 0x8226BB1C;
	}
	return 0x8226BB1C;
} // Block from 8226BAECh-8226BB1Ch (12 instructions)

//////////////////////////////////////////////////////
// Block at 8226BB1Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BB1C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BB1C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BB1C);
		  /* 8226BB1Ch */ case    0:  		/* mr R3, R31 */
		/* 8226BB1Ch case    0:*/		regs.R3 = regs.R31;
		/* 8226BB1Ch case    0:*/		return 0x8226BB20;
		  /* 8226BB20h */ case    1:  		/* bl -3296 */
		/* 8226BB20h case    1:*/		regs.LR = 0x8226BB24; return 0x8226AE40;
		/* 8226BB20h case    1:*/		return 0x8226BB24;
		  /* 8226BB24h */ case    2:  		/* lis R11, -32251 */
		/* 8226BB24h case    2:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BB24h case    2:*/		return 0x8226BB28;
		  /* 8226BB28h */ case    3:  		/* mr R3, R31 */
		/* 8226BB28h case    3:*/		regs.R3 = regs.R31;
		/* 8226BB28h case    3:*/		return 0x8226BB2C;
		  /* 8226BB2Ch */ case    4:  		/* addi R4, R11, -16128 */
		/* 8226BB2Ch case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC100);
		/* 8226BB2Ch case    4:*/		return 0x8226BB30;
		  /* 8226BB30h */ case    5:  		/* lwz R11, <#[R29 + 8]> */
		/* 8226BB30h case    5:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000008) );
		/* 8226BB30h case    5:*/		return 0x8226BB34;
		  /* 8226BB34h */ case    6:  		/* rlwinm R11, R11, 1, 0, 30 */
		/* 8226BB34h case    6:*/		cpu::op::rlwinm<0,1,0,30>(regs,&regs.R11,regs.R11);
		/* 8226BB34h case    6:*/		return 0x8226BB38;
		  /* 8226BB38h */ case    7:  		/* srawi R5, R11, 9 */
		/* 8226BB38h case    7:*/		cpu::op::srawi<0,9>(regs,&regs.R5,regs.R11);
		/* 8226BB38h case    7:*/		return 0x8226BB3C;
		  /* 8226BB3Ch */ case    8:  		/* bl -8540 */
		/* 8226BB3Ch case    8:*/		regs.LR = 0x8226BB40; return 0x822699E0;
		/* 8226BB3Ch case    8:*/		return 0x8226BB40;
	}
	return 0x8226BB40;
} // Block from 8226BB1Ch-8226BB40h (9 instructions)

//////////////////////////////////////////////////////
// Block at 8226BB40h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BB40( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BB40) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BB40);
		  /* 8226BB40h */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BB40h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BB40h case    0:*/		return 0x8226BB44;
		  /* 8226BB44h */ case    1:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BB44h case    1:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BB44h case    1:*/		return 0x8226BB48;
		  /* 8226BB48h */ case    2:  		/* bc 4, CR0_EQ, 16 */
		/* 8226BB48h case    2:*/		if ( !regs.CR[0].eq ) { return 0x8226BB58;  }
		/* 8226BB48h case    2:*/		return 0x8226BB4C;
		  /* 8226BB4Ch */ case    3:  		/* lhz R11, <#[R29 + 4]> */
		/* 8226BB4Ch case    3:*/		cpu::mem::load16z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BB4Ch case    3:*/		return 0x8226BB50;
		  /* 8226BB50h */ case    4:  		/* rlwinm. R11, R11, 0, 26, 31 */
		/* 8226BB50h case    4:*/		cpu::op::rlwinm<1,0,26,31>(regs,&regs.R11,regs.R11);
		/* 8226BB50h case    4:*/		return 0x8226BB54;
		  /* 8226BB54h */ case    5:  		/* bc 12, CR0_EQ, 44 */
		/* 8226BB54h case    5:*/		if ( regs.CR[0].eq ) { return 0x8226BB80;  }
		/* 8226BB54h case    5:*/		return 0x8226BB58;
	}
	return 0x8226BB58;
} // Block from 8226BB40h-8226BB58h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226BB58h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BB58( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BB58) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BB58);
		  /* 8226BB58h */ case    0:  		/* mr R3, R31 */
		/* 8226BB58h case    0:*/		regs.R3 = regs.R31;
		/* 8226BB58h case    0:*/		return 0x8226BB5C;
		  /* 8226BB5Ch */ case    1:  		/* bl -3356 */
		/* 8226BB5Ch case    1:*/		regs.LR = 0x8226BB60; return 0x8226AE40;
		/* 8226BB5Ch case    1:*/		return 0x8226BB60;
		  /* 8226BB60h */ case    2:  		/* lis R10, -32251 */
		/* 8226BB60h case    2:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8205);
		/* 8226BB60h case    2:*/		return 0x8226BB64;
		  /* 8226BB64h */ case    3:  		/* addi R11, R28, 528 */
		/* 8226BB64h case    3:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R28,0x210);
		/* 8226BB64h case    3:*/		return 0x8226BB68;
		  /* 8226BB68h */ case    4:  		/* addi R4, R10, -16144 */
		/* 8226BB68h case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R10,0xFFFFC0F0);
		/* 8226BB68h case    4:*/		return 0x8226BB6C;
		  /* 8226BB6Ch */ case    5:  		/* mr R3, R31 */
		/* 8226BB6Ch case    5:*/		regs.R3 = regs.R31;
		/* 8226BB6Ch case    5:*/		return 0x8226BB70;
		  /* 8226BB70h */ case    6:  		/* lhz R10, <#[R29 + 4]> */
		/* 8226BB70h case    6:*/		cpu::mem::load16z( regs, &regs.R10, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BB70h case    6:*/		return 0x8226BB74;
		  /* 8226BB74h */ case    7:  		/* rlwinm R10, R10, 2, 24, 29 */
		/* 8226BB74h case    7:*/		cpu::op::rlwinm<0,2,24,29>(regs,&regs.R10,regs.R10);
		/* 8226BB74h case    7:*/		return 0x8226BB78;
		  /* 8226BB78h */ case    8:  		/* lwzx R5, <#[R10 + R11]> */
		/* 8226BB78h case    8:*/		cpu::mem::load32z( regs, &regs.R5, (uint32)(regs.R10 + regs.R11 + 0x00000000) );
		/* 8226BB78h case    8:*/		return 0x8226BB7C;
		  /* 8226BB7Ch */ case    9:  		/* bl -8604 */
		/* 8226BB7Ch case    9:*/		regs.LR = 0x8226BB80; return 0x822699E0;
		/* 8226BB7Ch case    9:*/		return 0x8226BB80;
	}
	return 0x8226BB80;
} // Block from 8226BB58h-8226BB80h (10 instructions)

//////////////////////////////////////////////////////
// Block at 8226BB80h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BB80( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BB80) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BB80);
		  /* 8226BB80h */ case    0:  		/* cmplwi CR6, R24, 0 */
		/* 8226BB80h case    0:*/		cpu::op::cmplwi<6>(regs,regs.R24,0x00000000);
		/* 8226BB80h case    0:*/		return 0x8226BB84;
		  /* 8226BB84h */ case    1:  		/* bc 12, CR6_EQ, 56 */
		/* 8226BB84h case    1:*/		if ( regs.CR[6].eq ) { return 0x8226BBBC;  }
		/* 8226BB84h case    1:*/		return 0x8226BB88;
		  /* 8226BB88h */ case    2:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BB88h case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BB88h case    2:*/		return 0x8226BB8C;
		  /* 8226BB8Ch */ case    3:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BB8Ch case    3:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BB8Ch case    3:*/		return 0x8226BB90;
		  /* 8226BB90h */ case    4:  		/* bc 4, CR0_EQ, 16 */
		/* 8226BB90h case    4:*/		if ( !regs.CR[0].eq ) { return 0x8226BBA0;  }
		/* 8226BB90h case    4:*/		return 0x8226BB94;
		  /* 8226BB94h */ case    5:  		/* lbz R11, <#[R29 + 11]> */
		/* 8226BB94h case    5:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R29 + 0x0000000B) );
		/* 8226BB94h case    5:*/		return 0x8226BB98;
		  /* 8226BB98h */ case    6:  		/* cmplwi CR0, R11, 0 */
		/* 8226BB98h case    6:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BB98h case    6:*/		return 0x8226BB9C;
		  /* 8226BB9Ch */ case    7:  		/* bc 12, CR0_EQ, 32 */
		/* 8226BB9Ch case    7:*/		if ( regs.CR[0].eq ) { return 0x8226BBBC;  }
		/* 8226BB9Ch case    7:*/		return 0x8226BBA0;
	}
	return 0x8226BBA0;
} // Block from 8226BB80h-8226BBA0h (8 instructions)

//////////////////////////////////////////////////////
// Block at 8226BBA0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BBA0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BBA0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BBA0);
		  /* 8226BBA0h */ case    0:  		/* mr R3, R31 */
		/* 8226BBA0h case    0:*/		regs.R3 = regs.R31;
		/* 8226BBA0h case    0:*/		return 0x8226BBA4;
		  /* 8226BBA4h */ case    1:  		/* bl -3428 */
		/* 8226BBA4h case    1:*/		regs.LR = 0x8226BBA8; return 0x8226AE40;
		/* 8226BBA4h case    1:*/		return 0x8226BBA8;
		  /* 8226BBA8h */ case    2:  		/* lis R11, -32251 */
		/* 8226BBA8h case    2:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BBA8h case    2:*/		return 0x8226BBAC;
		  /* 8226BBACh */ case    3:  		/* mr R3, R31 */
		/* 8226BBACh case    3:*/		regs.R3 = regs.R31;
		/* 8226BBACh case    3:*/		return 0x8226BBB0;
		  /* 8226BBB0h */ case    4:  		/* lbz R5, <#[R29 + 11]> */
		/* 8226BBB0h case    4:*/		cpu::mem::load8z( regs, &regs.R5, (uint32)(regs.R29 + 0x0000000B) );
		/* 8226BBB0h case    4:*/		return 0x8226BBB4;
		  /* 8226BBB4h */ case    5:  		/* addi R4, R11, -16156 */
		/* 8226BBB4h case    5:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC0E4);
		/* 8226BBB4h case    5:*/		return 0x8226BBB8;
		  /* 8226BBB8h */ case    6:  		/* bl -8664 */
		/* 8226BBB8h case    6:*/		regs.LR = 0x8226BBBC; return 0x822699E0;
		/* 8226BBB8h case    6:*/		return 0x8226BBBC;
	}
	return 0x8226BBBC;
} // Block from 8226BBA0h-8226BBBCh (7 instructions)

//////////////////////////////////////////////////////
// Block at 8226BBBCh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BBBC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BBBC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BBBC);
		  /* 8226BBBCh */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BBBCh case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BBBCh case    0:*/		return 0x8226BBC0;
		  /* 8226BBC0h */ case    1:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BBC0h case    1:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BBC0h case    1:*/		return 0x8226BBC4;
		  /* 8226BBC4h */ case    2:  		/* bc 4, CR0_EQ, 16 */
		/* 8226BBC4h case    2:*/		if ( !regs.CR[0].eq ) { return 0x8226BBD4;  }
		/* 8226BBC4h case    2:*/		return 0x8226BBC8;
		  /* 8226BBC8h */ case    3:  		/* lwz R11, <#[R29 + 4]> */
		/* 8226BBC8h case    3:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BBC8h case    3:*/		return 0x8226BBCC;
		  /* 8226BBCCh */ case    4:  		/* rlwinm. R11, R11, 0, 19, 19 */
		/* 8226BBCCh case    4:*/		cpu::op::rlwinm<1,0,19,19>(regs,&regs.R11,regs.R11);
		/* 8226BBCCh case    4:*/		return 0x8226BBD0;
		  /* 8226BBD0h */ case    5:  		/* bc 12, CR0_EQ, 48 */
		/* 8226BBD0h case    5:*/		if ( regs.CR[0].eq ) { return 0x8226BC00;  }
		/* 8226BBD0h case    5:*/		return 0x8226BBD4;
	}
	return 0x8226BBD4;
} // Block from 8226BBBCh-8226BBD4h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226BBD4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BBD4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BBD4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BBD4);
		  /* 8226BBD4h */ case    0:  		/* mr R3, R31 */
		/* 8226BBD4h case    0:*/		regs.R3 = regs.R31;
		/* 8226BBD4h case    0:*/		return 0x8226BBD8;
		  /* 8226BBD8h */ case    1:  		/* bl -3480 */
		/* 8226BBD8h case    1:*/		regs.LR = 0x8226BBDC; return 0x8226AE40;
		/* 8226BBD8h case    1:*/		return 0x8226BBDC;
		  /* 8226BBDCh */ case    2:  		/* lwz R11, <#[R29 + 4]> */
		/* 8226BBDCh case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BBDCh case    2:*/		return 0x8226BBE0;
		  /* 8226BBE0h */ case    3:  		/* mr R5, R27 */
		/* 8226BBE0h case    3:*/		regs.R5 = regs.R27;
		/* 8226BBE0h case    3:*/		return 0x8226BBE4;
		  /* 8226BBE4h */ case    4:  		/* rlwinm. R11, R11, 0, 19, 19 */
		/* 8226BBE4h case    4:*/		cpu::op::rlwinm<1,0,19,19>(regs,&regs.R11,regs.R11);
		/* 8226BBE4h case    4:*/		return 0x8226BBE8;
		  /* 8226BBE8h */ case    5:  		/* bc 4, CR0_EQ, 8 */
		/* 8226BBE8h case    5:*/		if ( !regs.CR[0].eq ) { return 0x8226BBF0;  }
		/* 8226BBE8h case    5:*/		return 0x8226BBEC;
		  /* 8226BBECh */ case    6:  		/* mr R5, R30 */
		/* 8226BBECh case    6:*/		regs.R5 = regs.R30;
		/* 8226BBECh case    6:*/		return 0x8226BBF0;
	}
	return 0x8226BBF0;
} // Block from 8226BBD4h-8226BBF0h (7 instructions)

//////////////////////////////////////////////////////
// Block at 8226BBF0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BBF0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BBF0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BBF0);
		  /* 8226BBF0h */ case    0:  		/* lis R11, -32251 */
		/* 8226BBF0h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BBF0h case    0:*/		return 0x8226BBF4;
		  /* 8226BBF4h */ case    1:  		/* mr R3, R31 */
		/* 8226BBF4h case    1:*/		regs.R3 = regs.R31;
		/* 8226BBF4h case    1:*/		return 0x8226BBF8;
		  /* 8226BBF8h */ case    2:  		/* addi R4, R11, -16168 */
		/* 8226BBF8h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC0D8);
		/* 8226BBF8h case    2:*/		return 0x8226BBFC;
		  /* 8226BBFCh */ case    3:  		/* bl -8732 */
		/* 8226BBFCh case    3:*/		regs.LR = 0x8226BC00; return 0x822699E0;
		/* 8226BBFCh case    3:*/		return 0x8226BC00;
	}
	return 0x8226BC00;
} // Block from 8226BBF0h-8226BC00h (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226BC00h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BC00( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BC00) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BC00);
		  /* 8226BC00h */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BC00h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BC00h case    0:*/		return 0x8226BC04;
		  /* 8226BC04h */ case    1:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BC04h case    1:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BC04h case    1:*/		return 0x8226BC08;
		  /* 8226BC08h */ case    2:  		/* bc 4, CR0_EQ, 16 */
		/* 8226BC08h case    2:*/		if ( !regs.CR[0].eq ) { return 0x8226BC18;  }
		/* 8226BC08h case    2:*/		return 0x8226BC0C;
		  /* 8226BC0Ch */ case    3:  		/* lwz R11, <#[R29 + 4]> */
		/* 8226BC0Ch case    3:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BC0Ch case    3:*/		return 0x8226BC10;
		  /* 8226BC10h */ case    4:  		/* rlwinm. R11, R11, 0, 18, 18 */
		/* 8226BC10h case    4:*/		cpu::op::rlwinm<1,0,18,18>(regs,&regs.R11,regs.R11);
		/* 8226BC10h case    4:*/		return 0x8226BC14;
		  /* 8226BC14h */ case    5:  		/* bc 12, CR0_EQ, 60 */
		/* 8226BC14h case    5:*/		if ( regs.CR[0].eq ) { return 0x8226BC50;  }
		/* 8226BC14h case    5:*/		return 0x8226BC18;
	}
	return 0x8226BC18;
} // Block from 8226BC00h-8226BC18h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226BC18h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BC18( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BC18) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BC18);
		  /* 8226BC18h */ case    0:  		/* mr R3, R31 */
		/* 8226BC18h case    0:*/		regs.R3 = regs.R31;
		/* 8226BC18h case    0:*/		return 0x8226BC1C;
		  /* 8226BC1Ch */ case    1:  		/* bl -3548 */
		/* 8226BC1Ch case    1:*/		regs.LR = 0x8226BC20; return 0x8226AE40;
		/* 8226BC1Ch case    1:*/		return 0x8226BC20;
		  /* 8226BC20h */ case    2:  		/* lwz R11, <#[R29 + 4]> */
		/* 8226BC20h case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000004) );
		/* 8226BC20h case    2:*/		return 0x8226BC24;
		  /* 8226BC24h */ case    3:  		/* rlwinm. R11, R11, 0, 18, 18 */
		/* 8226BC24h case    3:*/		cpu::op::rlwinm<1,0,18,18>(regs,&regs.R11,regs.R11);
		/* 8226BC24h case    3:*/		return 0x8226BC28;
		  /* 8226BC28h */ case    4:  		/* bc 12, CR0_EQ, 16 */
		/* 8226BC28h case    4:*/		if ( regs.CR[0].eq ) { return 0x8226BC38;  }
		/* 8226BC28h case    4:*/		return 0x8226BC2C;
		  /* 8226BC2Ch */ case    5:  		/* lis R11, -32255 */
		/* 8226BC2Ch case    5:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BC2Ch case    5:*/		return 0x8226BC30;
		  /* 8226BC30h */ case    6:  		/* addi R5, R11, 11984 */
		/* 8226BC30h case    6:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x2ED0);
		/* 8226BC30h case    6:*/		return 0x8226BC34;
		  /* 8226BC34h */ case    7:  		/* b 12 */
		/* 8226BC34h case    7:*/		return 0x8226BC40;
		/* 8226BC34h case    7:*/		return 0x8226BC38;
	}
	return 0x8226BC38;
} // Block from 8226BC18h-8226BC38h (8 instructions)

//////////////////////////////////////////////////////
// Block at 8226BC38h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BC38( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BC38) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BC38);
		  /* 8226BC38h */ case    0:  		/* lis R11, -32255 */
		/* 8226BC38h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BC38h case    0:*/		return 0x8226BC3C;
		  /* 8226BC3Ch */ case    1:  		/* addi R5, R11, 11992 */
		/* 8226BC3Ch case    1:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x2ED8);
		/* 8226BC3Ch case    1:*/		return 0x8226BC40;
	}
	return 0x8226BC40;
} // Block from 8226BC38h-8226BC40h (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226BC40h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BC40( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BC40) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BC40);
		  /* 8226BC40h */ case    0:  		/* lis R11, -32251 */
		/* 8226BC40h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BC40h case    0:*/		return 0x8226BC44;
		  /* 8226BC44h */ case    1:  		/* mr R3, R31 */
		/* 8226BC44h case    1:*/		regs.R3 = regs.R31;
		/* 8226BC44h case    1:*/		return 0x8226BC48;
		  /* 8226BC48h */ case    2:  		/* addi R4, R11, -16184 */
		/* 8226BC48h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC0C8);
		/* 8226BC48h case    2:*/		return 0x8226BC4C;
		  /* 8226BC4Ch */ case    3:  		/* bl -8812 */
		/* 8226BC4Ch case    3:*/		regs.LR = 0x8226BC50; return 0x822699E0;
		/* 8226BC4Ch case    3:*/		return 0x8226BC50;
	}
	return 0x8226BC50;
} // Block from 8226BC40h-8226BC50h (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226BC50h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BC50( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BC50) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BC50);
		  /* 8226BC50h */ case    0:  		/* cmplwi CR6, R24, 0 */
		/* 8226BC50h case    0:*/		cpu::op::cmplwi<6>(regs,regs.R24,0x00000000);
		/* 8226BC50h case    0:*/		return 0x8226BC54;
		  /* 8226BC54h */ case    1:  		/* bc 12, CR6_EQ, 64 */
		/* 8226BC54h case    1:*/		if ( regs.CR[6].eq ) { return 0x8226BC94;  }
		/* 8226BC54h case    1:*/		return 0x8226BC58;
		  /* 8226BC58h */ case    2:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BC58h case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BC58h case    2:*/		return 0x8226BC5C;
		  /* 8226BC5Ch */ case    3:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226BC5Ch case    3:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226BC5Ch case    3:*/		return 0x8226BC60;
		  /* 8226BC60h */ case    4:  		/* bc 4, CR0_EQ, 16 */
		/* 8226BC60h case    4:*/		if ( !regs.CR[0].eq ) { return 0x8226BC70;  }
		/* 8226BC60h case    4:*/		return 0x8226BC64;
		  /* 8226BC64h */ case    5:  		/* lwz R11, <#[R29]> */
		/* 8226BC64h case    5:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000000) );
		/* 8226BC64h case    5:*/		return 0x8226BC68;
		  /* 8226BC68h */ case    6:  		/* rlwinm. R11, R11, 0, 2, 4 */
		/* 8226BC68h case    6:*/		cpu::op::rlwinm<1,0,2,4>(regs,&regs.R11,regs.R11);
		/* 8226BC68h case    6:*/		return 0x8226BC6C;
		  /* 8226BC6Ch */ case    7:  		/* bc 12, CR0_EQ, 40 */
		/* 8226BC6Ch case    7:*/		if ( regs.CR[0].eq ) { return 0x8226BC94;  }
		/* 8226BC6Ch case    7:*/		return 0x8226BC70;
	}
	return 0x8226BC70;
} // Block from 8226BC50h-8226BC70h (8 instructions)

//////////////////////////////////////////////////////
// Block at 8226BC70h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BC70( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BC70) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BC70);
		  /* 8226BC70h */ case    0:  		/* mr R3, R31 */
		/* 8226BC70h case    0:*/		regs.R3 = regs.R31;
		/* 8226BC70h case    0:*/		return 0x8226BC74;
		  /* 8226BC74h */ case    1:  		/* bl -3636 */
		/* 8226BC74h case    1:*/		regs.LR = 0x8226BC78; return 0x8226AE40;
		/* 8226BC74h case    1:*/		return 0x8226BC78;
		  /* 8226BC78h */ case    2:  		/* lis R11, -32251 */
		/* 8226BC78h case    2:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BC78h case    2:*/		return 0x8226BC7C;
		  /* 8226BC7Ch */ case    3:  		/* mr R3, R31 */
		/* 8226BC7Ch case    3:*/		regs.R3 = regs.R31;
		/* 8226BC7Ch case    3:*/		return 0x8226BC80;
		  /* 8226BC80h */ case    4:  		/* addi R4, R11, -16204 */
		/* 8226BC80h case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC0B4);
		/* 8226BC80h case    4:*/		return 0x8226BC84;
		  /* 8226BC84h */ case    5:  		/* lwz R11, <#[R29]> */
		/* 8226BC84h case    5:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R29 + 0x00000000) );
		/* 8226BC84h case    5:*/		return 0x8226BC88;
		  /* 8226BC88h */ case    6:  		/* rlwinm R11, R11, 5, 29, 31 */
		/* 8226BC88h case    6:*/		cpu::op::rlwinm<0,5,29,31>(regs,&regs.R11,regs.R11);
		/* 8226BC88h case    6:*/		return 0x8226BC8C;
		  /* 8226BC8Ch */ case    7:  		/* addi R5, R11, 1 */
		/* 8226BC8Ch case    7:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x1);
		/* 8226BC8Ch case    7:*/		return 0x8226BC90;
		  /* 8226BC90h */ case    8:  		/* bl -8880 */
		/* 8226BC90h case    8:*/		regs.LR = 0x8226BC94; return 0x822699E0;
		/* 8226BC90h case    8:*/		return 0x8226BC94;
	}
	return 0x8226BC94;
} // Block from 8226BC70h-8226BC94h (9 instructions)

//////////////////////////////////////////////////////
// Block at 8226BC94h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BC94( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BC94) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BC94);
		  /* 8226BC94h */ case    0:  		/* lis R11, -32253 */
		/* 8226BC94h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8203);
		/* 8226BC94h case    0:*/		return 0x8226BC98;
		  /* 8226BC98h */ case    1:  		/* mr R3, R31 */
		/* 8226BC98h case    1:*/		regs.R3 = regs.R31;
		/* 8226BC98h case    1:*/		return 0x8226BC9C;
		  /* 8226BC9Ch */ case    2:  		/* addi R4, R11, 28348 */
		/* 8226BC9Ch case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x6EBC);
		/* 8226BC9Ch case    2:*/		return 0x8226BCA0;
		  /* 8226BCA0h */ case    3:  		/* bl -8632 */
		/* 8226BCA0h case    3:*/		regs.LR = 0x8226BCA4; return 0x82269AE8;
		/* 8226BCA0h case    3:*/		return 0x8226BCA4;
	}
	return 0x8226BCA4;
} // Block from 8226BC94h-8226BCA4h (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226BCA4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BCA4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BCA4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BCA4);
		  /* 8226BCA4h */ case    0:  		/* addi R1, R1, 224 */
		/* 8226BCA4h case    0:*/		cpu::op::addi<0>(regs,&regs.R1,regs.R1,0xE0);
		/* 8226BCA4h case    0:*/		return 0x8226BCA8;
		  /* 8226BCA8h */ case    1:  		/* lfd FR31, <#[R1 - 88]> */
		/* 8226BCA8h case    1:*/		cpu::mem::load64f( regs, &regs.FR31, (uint32)(regs.R1 + 0xFFFFFFA8) );
		/* 8226BCA8h case    1:*/		return 0x8226BCAC;
		  /* 8226BCACh */ case    2:  		/* b -1944088 */
		/* 8226BCACh case    2:*/		return 0x82091294;
		/* 8226BCACh case    2:*/		return 0x8226BCB0;
	}
	return 0x8226BCB0;
} // Block from 8226BCA4h-8226BCB0h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226BCB0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BCB0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BCB0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BCB0);
		  /* 8226BCB0h */ case    0:  		/* mfspr R12, LR */
		/* 8226BCB0h case    0:*/		regs.R12 = regs.LR;
		/* 8226BCB0h case    0:*/		return 0x8226BCB4;
		  /* 8226BCB4h */ case    1:  		/* bl -1944184 */
		/* 8226BCB4h case    1:*/		regs.LR = 0x8226BCB8; return 0x8209123C;
		/* 8226BCB4h case    1:*/		return 0x8226BCB8;
		  /* 8226BCB8h */ case    2:  		/* stwu R1, <#[R1 - 208]> */
		/* 8226BCB8h case    2:*/		cpu::mem::store32( regs, regs.R1, (uint32)(regs.R1 + 0xFFFFFF30) );
		regs.R1 = (uint32)(regs.R1 + 0xFFFFFF30);
		/* 8226BCB8h case    2:*/		return 0x8226BCBC;
		  /* 8226BCBCh */ case    3:  		/* mr R31, R3 */
		/* 8226BCBCh case    3:*/		regs.R31 = regs.R3;
		/* 8226BCBCh case    3:*/		return 0x8226BCC0;
		  /* 8226BCC0h */ case    4:  		/* mr R29, R4 */
		/* 8226BCC0h case    4:*/		regs.R29 = regs.R4;
		/* 8226BCC0h case    4:*/		return 0x8226BCC4;
		  /* 8226BCC4h */ case    5:  		/* mr R22, R5 */
		/* 8226BCC4h case    5:*/		regs.R22 = regs.R5;
		/* 8226BCC4h case    5:*/		return 0x8226BCC8;
		  /* 8226BCC8h */ case    6:  		/* mr R28, R6 */
		/* 8226BCC8h case    6:*/		regs.R28 = regs.R6;
		/* 8226BCC8h case    6:*/		return 0x8226BCCC;
		  /* 8226BCCCh */ case    7:  		/* rlwinm. R11, R4, 0, 31, 31 */
		/* 8226BCCCh case    7:*/		cpu::op::rlwinm<1,0,31,31>(regs,&regs.R11,regs.R4);
		/* 8226BCCCh case    7:*/		return 0x8226BCD0;
		  /* 8226BCD0h */ case    8:  		/* bc 12, CR0_EQ, 2180 */
		/* 8226BCD0h case    8:*/		if ( regs.CR[0].eq ) { return 0x8226C554;  }
		/* 8226BCD0h case    8:*/		return 0x8226BCD4;
		  /* 8226BCD4h */ case    9:  		/* lwz R11, <#[R6 + 4]> */
		/* 8226BCD4h case    9:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R6 + 0x00000004) );
		/* 8226BCD4h case    9:*/		return 0x8226BCD8;
		  /* 8226BCD8h */ case   10:  		/* lbz R10, <#[R3 + 12]> */
		/* 8226BCD8h case   10:*/		cpu::mem::load8z( regs, &regs.R10, (uint32)(regs.R3 + 0x0000000C) );
		/* 8226BCD8h case   10:*/		return 0x8226BCDC;
		  /* 8226BCDCh */ case   11:  		/* rlwinm R30, R11, 20, 28, 31 */
		/* 8226BCDCh case   11:*/		cpu::op::rlwinm<0,20,28,31>(regs,&regs.R30,regs.R11);
		/* 8226BCDCh case   11:*/		return 0x8226BCE0;
		  /* 8226BCE0h */ case   12:  		/* cmplwi CR0, R10, 0 */
		/* 8226BCE0h case   12:*/		cpu::op::cmplwi<0>(regs,regs.R10,0x00000000);
		/* 8226BCE0h case   12:*/		return 0x8226BCE4;
		  /* 8226BCE4h */ case   13:  		/* bc 12, CR0_EQ, 160 */
		/* 8226BCE4h case   13:*/		if ( regs.CR[0].eq ) { return 0x8226BD84;  }
		/* 8226BCE4h case   13:*/		return 0x8226BCE8;
		  /* 8226BCE8h */ case   14:  		/* bl -4136 */
		/* 8226BCE8h case   14:*/		regs.LR = 0x8226BCEC; return 0x8226ACC0;
		/* 8226BCE8h case   14:*/		return 0x8226BCEC;
		  /* 8226BCECh */ case   15:  		/* rlwinm R11, R22, 27, 5, 31 */
		/* 8226BCECh case   15:*/		cpu::op::rlwinm<0,27,5,31>(regs,&regs.R11,regs.R22);
		/* 8226BCECh case   15:*/		return 0x8226BCF0;
		  /* 8226BCF0h */ case   16:  		/* rlwinm R10, R22, 0, 27, 31 */
		/* 8226BCF0h case   16:*/		cpu::op::rlwinm<0,0,27,31>(regs,&regs.R10,regs.R22);
		/* 8226BCF0h case   16:*/		return 0x8226BCF4;
		  /* 8226BCF4h */ case   17:  		/* addi R11, R11, 6 */
		/* 8226BCF4h case   17:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R11,0x6);
		/* 8226BCF4h case   17:*/		return 0x8226BCF8;
		  /* 8226BCF8h */ case   18:  		/* li R9, 1 */
		/* 8226BCF8h case   18:*/		cpu::op::li<0>(regs,&regs.R9,0x1);
		/* 8226BCF8h case   18:*/		return 0x8226BCFC;
		  /* 8226BCFCh */ case   19:  		/* rlwinm R11, R11, 2, 0, 29 */
		/* 8226BCFCh case   19:*/		cpu::op::rlwinm<0,2,0,29>(regs,&regs.R11,regs.R11);
		/* 8226BCFCh case   19:*/		return 0x8226BD00;
		  /* 8226BD00h */ case   20:  		/* slw R10, R9, R10 */
		/* 8226BD00h case   20:*/		cpu::op::slw<0>(regs,&regs.R10,regs.R9,regs.R10);
		/* 8226BD00h case   20:*/		return 0x8226BD04;
		  /* 8226BD04h */ case   21:  		/* lwzx R11, <#[R11 + R31]> */
		/* 8226BD04h case   21:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R11 + regs.R31 + 0x00000000) );
		/* 8226BD04h case   21:*/		return 0x8226BD08;
		  /* 8226BD08h */ case   22:  		/* and. R11, R10, R11 */
		/* 8226BD08h case   22:*/		cpu::op::and<1>(regs,&regs.R11,regs.R10,regs.R11);
		/* 8226BD08h case   22:*/		return 0x8226BD0C;
		  /* 8226BD0Ch */ case   23:  		/* bc 12, CR0_EQ, 88 */
		/* 8226BD0Ch case   23:*/		if ( regs.CR[0].eq ) { return 0x8226BD64;  }
		/* 8226BD0Ch case   23:*/		return 0x8226BD10;
		  /* 8226BD10h */ case   24:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BD10h case   24:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BD10h case   24:*/		return 0x8226BD14;
		  /* 8226BD14h */ case   25:  		/* rlwinm. R11, R11, 0, 26, 26 */
		/* 8226BD14h case   25:*/		cpu::op::rlwinm<1,0,26,26>(regs,&regs.R11,regs.R11);
		/* 8226BD14h case   25:*/		return 0x8226BD18;
		  /* 8226BD18h */ case   26:  		/* bc 12, CR0_EQ, 20 */
		/* 8226BD18h case   26:*/		if ( regs.CR[0].eq ) { return 0x8226BD2C;  }
		/* 8226BD18h case   26:*/		return 0x8226BD1C;
		  /* 8226BD1Ch */ case   27:  		/* lis R11, -32251 */
		/* 8226BD1Ch case   27:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BD1Ch case   27:*/		return 0x8226BD20;
		  /* 8226BD20h */ case   28:  		/* mr R3, R31 */
		/* 8226BD20h case   28:*/		regs.R3 = regs.R31;
		/* 8226BD20h case   28:*/		return 0x8226BD24;
		  /* 8226BD24h */ case   29:  		/* addi R4, R11, -15820 */
		/* 8226BD24h case   29:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC234);
		/* 8226BD24h case   29:*/		return 0x8226BD28;
		  /* 8226BD28h */ case   30:  		/* bl -9032 */
		/* 8226BD28h case   30:*/		regs.LR = 0x8226BD2C; return 0x822699E0;
		/* 8226BD28h case   30:*/		return 0x8226BD2C;
	}
	return 0x8226BD2C;
} // Block from 8226BCB0h-8226BD2Ch (31 instructions)

//////////////////////////////////////////////////////
// Block at 8226BD2Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BD2C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BD2C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BD2C);
		  /* 8226BD2Ch */ case    0:  		/* lis R11, -32255 */
		/* 8226BD2Ch case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BD2Ch case    0:*/		return 0x8226BD30;
		  /* 8226BD30h */ case    1:  		/* mr R3, R31 */
		/* 8226BD30h case    1:*/		regs.R3 = regs.R31;
		/* 8226BD30h case    1:*/		return 0x8226BD34;
		  /* 8226BD34h */ case    2:  		/* addi R4, R11, -8312 */
		/* 8226BD34h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFDF88);
		/* 8226BD34h case    2:*/		return 0x8226BD38;
		  /* 8226BD38h */ case    3:  		/* bl -8584 */
		/* 8226BD38h case    3:*/		regs.LR = 0x8226BD3C; return 0x82269BB0;
		/* 8226BD38h case    3:*/		return 0x8226BD3C;
		  /* 8226BD3Ch */ case    4:  		/* lis R11, -32251 */
		/* 8226BD3Ch case    4:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BD3Ch case    4:*/		return 0x8226BD40;
		  /* 8226BD40h */ case    5:  		/* mr R5, R22 */
		/* 8226BD40h case    5:*/		regs.R5 = regs.R22;
		/* 8226BD40h case    5:*/		return 0x8226BD44;
		  /* 8226BD44h */ case    6:  		/* addi R4, R11, -15828 */
		/* 8226BD44h case    6:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC22C);
		/* 8226BD44h case    6:*/		return 0x8226BD48;
		  /* 8226BD48h */ case    7:  		/* mr R3, R31 */
		/* 8226BD48h case    7:*/		regs.R3 = regs.R31;
		/* 8226BD48h case    7:*/		return 0x8226BD4C;
		  /* 8226BD4Ch */ case    8:  		/* bl -9068 */
		/* 8226BD4Ch case    8:*/		regs.LR = 0x8226BD50; return 0x822699E0;
		/* 8226BD4Ch case    8:*/		return 0x8226BD50;
		  /* 8226BD50h */ case    9:  		/* lbz R11, <#[R31 + 8232]> */
		/* 8226BD50h case    9:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x00002028) );
		/* 8226BD50h case    9:*/		return 0x8226BD54;
		  /* 8226BD54h */ case   10:  		/* cmplwi CR0, R11, 0 */
		/* 8226BD54h case   10:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BD54h case   10:*/		return 0x8226BD58;
		  /* 8226BD58h */ case   11:  		/* bc 4, CR0_EQ, 12 */
		/* 8226BD58h case   11:*/		if ( !regs.CR[0].eq ) { return 0x8226BD64;  }
		/* 8226BD58h case   11:*/		return 0x8226BD5C;
		  /* 8226BD5Ch */ case   12:  		/* mr R3, R31 */
		/* 8226BD5Ch case   12:*/		regs.R3 = regs.R31;
		/* 8226BD5Ch case   12:*/		return 0x8226BD60;
		  /* 8226BD60h */ case   13:  		/* bl -4344 */
		/* 8226BD60h case   13:*/		regs.LR = 0x8226BD64; return 0x8226AC68;
		/* 8226BD60h case   13:*/		return 0x8226BD64;
	}
	return 0x8226BD64;
} // Block from 8226BD2Ch-8226BD64h (14 instructions)

//////////////////////////////////////////////////////
// Block at 8226BD64h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BD64( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BD64) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BD64);
		  /* 8226BD64h */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BD64h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BD64h case    0:*/		return 0x8226BD68;
		  /* 8226BD68h */ case    1:  		/* rlwinm. R11, R11, 0, 29, 29 */
		/* 8226BD68h case    1:*/		cpu::op::rlwinm<1,0,29,29>(regs,&regs.R11,regs.R11);
		/* 8226BD68h case    1:*/		return 0x8226BD6C;
		  /* 8226BD6Ch */ case    2:  		/* bc 4, CR0_EQ, 12 */
		/* 8226BD6Ch case    2:*/		if ( !regs.CR[0].eq ) { return 0x8226BD78;  }
		/* 8226BD6Ch case    2:*/		return 0x8226BD70;
		  /* 8226BD70h */ case    3:  		/* cmplwi CR6, R30, 0 */
		/* 8226BD70h case    3:*/		cpu::op::cmplwi<6>(regs,regs.R30,0x00000000);
		/* 8226BD70h case    3:*/		return 0x8226BD74;
		  /* 8226BD74h */ case    4:  		/* bc 12, CR6_EQ, 16 */
		/* 8226BD74h case    4:*/		if ( regs.CR[6].eq ) { return 0x8226BD84;  }
		/* 8226BD74h case    4:*/		return 0x8226BD78;
	}
	return 0x8226BD78;
} // Block from 8226BD64h-8226BD78h (5 instructions)

//////////////////////////////////////////////////////
// Block at 8226BD78h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BD78( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BD78) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BD78);
		  /* 8226BD78h */ case    0:  		/* mr R4, R22 */
		/* 8226BD78h case    0:*/		regs.R4 = regs.R22;
		/* 8226BD78h case    0:*/		return 0x8226BD7C;
		  /* 8226BD7Ch */ case    1:  		/* mr R3, R31 */
		/* 8226BD7Ch case    1:*/		regs.R3 = regs.R31;
		/* 8226BD7Ch case    1:*/		return 0x8226BD80;
		  /* 8226BD80h */ case    2:  		/* bl -8552 */
		/* 8226BD80h case    2:*/		regs.LR = 0x8226BD84; return 0x82269C18;
		/* 8226BD80h case    2:*/		return 0x8226BD84;
	}
	return 0x8226BD84;
} // Block from 8226BD78h-8226BD84h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226BD84h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BD84( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BD84) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BD84);
		  /* 8226BD84h */ case    0:  		/* cmplwi CR6, R30, 15 */
		/* 8226BD84h case    0:*/		cpu::op::cmplwi<6>(regs,regs.R30,0x0000000F);
		/* 8226BD84h case    0:*/		return 0x8226BD88;
		  /* 8226BD88h */ case    1:  		/* bc 12, CR6_GT, 1928 */
		/* 8226BD88h case    1:*/		if ( regs.CR[6].gt ) { return 0x8226C510;  }
		/* 8226BD88h case    1:*/		return 0x8226BD8C;
		  /* 8226BD8Ch */ case    2:  		/* lis R12, -32251 */
		/* 8226BD8Ch case    2:*/		cpu::op::lis<0>(regs,&regs.R12,0xFFFF8205);
		/* 8226BD8Ch case    2:*/		return 0x8226BD90;
		  /* 8226BD90h */ case    3:  		/* rlwinm R0, R30, 1, 0, 30 */
		/* 8226BD90h case    3:*/		cpu::op::rlwinm<0,1,0,30>(regs,&regs.R0,regs.R30);
		/* 8226BD90h case    3:*/		return 0x8226BD94;
		  /* 8226BD94h */ case    4:  		/* addi R12, R12, -16424 */
		/* 8226BD94h case    4:*/		cpu::op::addi<0>(regs,&regs.R12,regs.R12,0xFFFFBFD8);
		/* 8226BD94h case    4:*/		return 0x8226BD98;
		  /* 8226BD98h */ case    5:  		/* lhzx R0, <#[R12 + R0]> */
		/* 8226BD98h case    5:*/		cpu::mem::load16z( regs, &regs.R0, (uint32)(regs.R12 + regs.R0 + 0x00000000) );
		/* 8226BD98h case    5:*/		return 0x8226BD9C;
		  /* 8226BD9Ch */ case    6:  		/* lis R12, -32217 */
		/* 8226BD9Ch case    6:*/		cpu::op::lis<0>(regs,&regs.R12,0xFFFF8227);
		/* 8226BD9Ch case    6:*/		return 0x8226BDA0;
		  /* 8226BDA0h */ case    7:  		/* addi R12, R12, -16972 */
		/* 8226BDA0h case    7:*/		cpu::op::addi<0>(regs,&regs.R12,regs.R12,0xFFFFBDB4);
		/* 8226BDA0h case    7:*/		return 0x8226BDA4;
		  /* 8226BDA4h */ case    8:  		/* ori R0, R0, 0 */
		/* 8226BDA4h case    8:*/		cpu::op::ori<0>(regs,&regs.R0,regs.R0,0x0);
		/* 8226BDA4h case    8:*/		return 0x8226BDA8;
		  /* 8226BDA8h */ case    9:  		/* add R12, R12, R0 */
		/* 8226BDA8h case    9:*/		cpu::op::add<0>(regs,&regs.R12,regs.R12,regs.R0);
		/* 8226BDA8h case    9:*/		return 0x8226BDAC;
		  /* 8226BDACh */ case   10:  		/* mtspr CTR, R12 */
		/* 8226BDACh case   10:*/		regs.CTR = regs.R12;
		/* 8226BDACh case   10:*/		return 0x8226BDB0;
		  /* 8226BDB0h */ case   11:  		/* bcctr 20, CR0_LT */
		/* 8226BDB0h case   11:*/		if ( 1 ) { return (uint32)regs.CTR; }
		/* 8226BDB0h case   11:*/		return 0x8226BDB4;
		  /* 8226BDB4h */ case   12:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226BDB4h case   12:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226BDB4h case   12:*/		return 0x8226BDB8;
		  /* 8226BDB8h */ case   13:  		/* cmplwi CR0, R11, 0 */
		/* 8226BDB8h case   13:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BDB8h case   13:*/		return 0x8226BDBC;
		  /* 8226BDBCh */ case   14:  		/* bc 12, CR0_EQ, 2796 */
		/* 8226BDBCh case   14:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BDBCh case   14:*/		return 0x8226BDC0;
		  /* 8226BDC0h */ case   15:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BDC0h case   15:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BDC0h case   15:*/		return 0x8226BDC4;
		  /* 8226BDC4h */ case   16:  		/* rlwinm. R11, R11, 0, 29, 29 */
		/* 8226BDC4h case   16:*/		cpu::op::rlwinm<1,0,29,29>(regs,&regs.R11,regs.R11);
		/* 8226BDC4h case   16:*/		return 0x8226BDC8;
		  /* 8226BDC8h */ case   17:  		/* bc 12, CR0_EQ, 2784 */
		/* 8226BDC8h case   17:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BDC8h case   17:*/		return 0x8226BDCC;
		  /* 8226BDCCh */ case   18:  		/* lis R11, -32254 */
		/* 8226BDCCh case   18:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226BDCCh case   18:*/		return 0x8226BDD0;
		  /* 8226BDD0h */ case   19:  		/* mr R3, R31 */
		/* 8226BDD0h case   19:*/		regs.R3 = regs.R31;
		/* 8226BDD0h case   19:*/		return 0x8226BDD4;
		  /* 8226BDD4h */ case   20:  		/* addi R4, R11, -30880 */
		/* 8226BDD4h case   20:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226BDD4h case   20:*/		return 0x8226BDD8;
		  /* 8226BDD8h */ case   21:  		/* bl -9208 */
		/* 8226BDD8h case   21:*/		regs.LR = 0x8226BDDC; return 0x822699E0;
		/* 8226BDD8h case   21:*/		return 0x8226BDDC;
		  /* 8226BDDCh */ case   22:  		/* lis R11, -32255 */
		/* 8226BDDCh case   22:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BDDCh case   22:*/		return 0x8226BDE0;
		  /* 8226BDE0h */ case   23:  		/* addi R4, R11, 12944 */
		/* 8226BDE0h case   23:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3290);
		/* 8226BDE0h case   23:*/		return 0x8226BDE4;
		  /* 8226BDE4h */ case   24:  		/* mr R3, R31 */
		/* 8226BDE4h case   24:*/		regs.R3 = regs.R31;
		/* 8226BDE4h case   24:*/		return 0x8226BDE8;
		  /* 8226BDE8h */ case   25:  		/* bl -8760 */
		/* 8226BDE8h case   25:*/		regs.LR = 0x8226BDEC; return 0x82269BB0;
		/* 8226BDE8h case   25:*/		return 0x8226BDEC;
		  /* 8226BDECh */ case   26:  		/* b 2748 */
		/* 8226BDECh case   26:*/		return 0x8226C8A8;
		/* 8226BDECh case   26:*/		return 0x8226BDF0;
		  /* 8226BDF0h */ case   27:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226BDF0h case   27:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226BDF0h case   27:*/		return 0x8226BDF4;
		  /* 8226BDF4h */ case   28:  		/* cmplwi CR0, R11, 0 */
		/* 8226BDF4h case   28:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BDF4h case   28:*/		return 0x8226BDF8;
		  /* 8226BDF8h */ case   29:  		/* bc 12, CR0_EQ, 2736 */
		/* 8226BDF8h case   29:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BDF8h case   29:*/		return 0x8226BDFC;
		  /* 8226BDFCh */ case   30:  		/* lis R11, -32254 */
		/* 8226BDFCh case   30:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226BDFCh case   30:*/		return 0x8226BE00;
		  /* 8226BE00h */ case   31:  		/* addi R4, R11, -30880 */
		/* 8226BE00h case   31:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226BE00h case   31:*/		return 0x8226BE04;
		  /* 8226BE04h */ case   32:  		/* mr R3, R31 */
		/* 8226BE04h case   32:*/		regs.R3 = regs.R31;
		/* 8226BE04h case   32:*/		return 0x8226BE08;
		  /* 8226BE08h */ case   33:  		/* bl -9256 */
		/* 8226BE08h case   33:*/		regs.LR = 0x8226BE0C; return 0x822699E0;
		/* 8226BE08h case   33:*/		return 0x8226BE0C;
		  /* 8226BE0Ch */ case   34:  		/* lis R11, -32255 */
		/* 8226BE0Ch case   34:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BE0Ch case   34:*/		return 0x8226BE10;
		  /* 8226BE10h */ case   35:  		/* addi R4, R11, 12936 */
		/* 8226BE10h case   35:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3288);
		/* 8226BE10h case   35:*/		return 0x8226BE14;
		  /* 8226BE14h */ case   36:  		/* mr R3, R31 */
		/* 8226BE14h case   36:*/		regs.R3 = regs.R31;
		/* 8226BE14h case   36:*/		return 0x8226BE18;
		  /* 8226BE18h */ case   37:  		/* bl -8808 */
		/* 8226BE18h case   37:*/		regs.LR = 0x8226BE1C; return 0x82269BB0;
		/* 8226BE18h case   37:*/		return 0x8226BE1C;
		  /* 8226BE1Ch */ case   38:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226BE1Ch case   38:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226BE1Ch case   38:*/		return 0x8226BE20;
		  /* 8226BE20h */ case   39:  		/* lwz R10, <#[R28]> */
		/* 8226BE20h case   39:*/		cpu::mem::load32z( regs, &regs.R10, (uint32)(regs.R28 + 0x00000000) );
		/* 8226BE20h case   39:*/		return 0x8226BE24;
		  /* 8226BE24h */ case   40:  		/* li R6, 0 */
		/* 8226BE24h case   40:*/		cpu::op::li<0>(regs,&regs.R6,0x0);
		/* 8226BE24h case   40:*/		return 0x8226BE28;
		  /* 8226BE28h */ case   41:  		/* rlwinm R5, R11, 23, 31, 31 */
		/* 8226BE28h case   41:*/		cpu::op::rlwinm<0,23,31,31>(regs,&regs.R5,regs.R11);
		/* 8226BE28h case   41:*/		return 0x8226BE2C;
		  /* 8226BE2Ch */ case   42:  		/* rlwinm R4, R10, 17, 31, 31 */
		/* 8226BE2Ch case   42:*/		cpu::op::rlwinm<0,17,31,31>(regs,&regs.R4,regs.R10);
		/* 8226BE2Ch case   42:*/		return 0x8226BE30;
		  /* 8226BE30h */ case   43:  		/* mr R3, R31 */
		/* 8226BE30h case   43:*/		regs.R3 = regs.R31;
		/* 8226BE30h case   43:*/		return 0x8226BE34;
		  /* 8226BE34h */ case   44:  		/* bl -9212 */
		/* 8226BE34h case   44:*/		regs.LR = 0x8226BE38; return 0x82269A38;
		/* 8226BE34h case   44:*/		return 0x8226BE38;
		  /* 8226BE38h */ case   45:  		/* b 2672 */
		/* 8226BE38h case   45:*/		return 0x8226C8A8;
		/* 8226BE38h case   45:*/		return 0x8226BE3C;
		  /* 8226BE3Ch */ case   46:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226BE3Ch case   46:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226BE3Ch case   46:*/		return 0x8226BE40;
		  /* 8226BE40h */ case   47:  		/* cmplwi CR0, R11, 0 */
		/* 8226BE40h case   47:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BE40h case   47:*/		return 0x8226BE44;
		  /* 8226BE44h */ case   48:  		/* bc 12, CR0_EQ, 2660 */
		/* 8226BE44h case   48:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BE44h case   48:*/		return 0x8226BE48;
		  /* 8226BE48h */ case   49:  		/* lis R11, -32254 */
		/* 8226BE48h case   49:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226BE48h case   49:*/		return 0x8226BE4C;
		  /* 8226BE4Ch */ case   50:  		/* addi R4, R11, -30880 */
		/* 8226BE4Ch case   50:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226BE4Ch case   50:*/		return 0x8226BE50;
		  /* 8226BE50h */ case   51:  		/* mr R3, R31 */
		/* 8226BE50h case   51:*/		regs.R3 = regs.R31;
		/* 8226BE50h case   51:*/		return 0x8226BE54;
		  /* 8226BE54h */ case   52:  		/* bl -9332 */
		/* 8226BE54h case   52:*/		regs.LR = 0x8226BE58; return 0x822699E0;
		/* 8226BE54h case   52:*/		return 0x8226BE58;
		  /* 8226BE58h */ case   53:  		/* lis R11, -32255 */
		/* 8226BE58h case   53:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BE58h case   53:*/		return 0x8226BE5C;
		  /* 8226BE5Ch */ case   54:  		/* addi R4, R11, 12928 */
		/* 8226BE5Ch case   54:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3280);
		/* 8226BE5Ch case   54:*/		return 0x8226BE60;
		  /* 8226BE60h */ case   55:  		/* b -76 */
		/* 8226BE60h case   55:*/		return 0x8226BE14;
		/* 8226BE60h case   55:*/		return 0x8226BE64;
		  /* 8226BE64h */ case   56:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226BE64h case   56:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226BE64h case   56:*/		return 0x8226BE68;
		  /* 8226BE68h */ case   57:  		/* cmplwi CR0, R11, 0 */
		/* 8226BE68h case   57:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BE68h case   57:*/		return 0x8226BE6C;
		  /* 8226BE6Ch */ case   58:  		/* bc 12, CR0_EQ, 2620 */
		/* 8226BE6Ch case   58:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BE6Ch case   58:*/		return 0x8226BE70;
		  /* 8226BE70h */ case   59:  		/* lis R11, -32254 */
		/* 8226BE70h case   59:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226BE70h case   59:*/		return 0x8226BE74;
		  /* 8226BE74h */ case   60:  		/* mr R3, R31 */
		/* 8226BE74h case   60:*/		regs.R3 = regs.R31;
		/* 8226BE74h case   60:*/		return 0x8226BE78;
		  /* 8226BE78h */ case   61:  		/* addi R4, R11, -30880 */
		/* 8226BE78h case   61:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226BE78h case   61:*/		return 0x8226BE7C;
		  /* 8226BE7Ch */ case   62:  		/* bl -9372 */
		/* 8226BE7Ch case   62:*/		regs.LR = 0x8226BE80; return 0x822699E0;
		/* 8226BE7Ch case   62:*/		return 0x8226BE80;
		  /* 8226BE80h */ case   63:  		/* lis R11, -32255 */
		/* 8226BE80h case   63:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BE80h case   63:*/		return 0x8226BE84;
		  /* 8226BE84h */ case   64:  		/* addi R4, R11, 12920 */
		/* 8226BE84h case   64:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3278);
		/* 8226BE84h case   64:*/		return 0x8226BE88;
		  /* 8226BE88h */ case   65:  		/* mr R3, R31 */
		/* 8226BE88h case   65:*/		regs.R3 = regs.R31;
		/* 8226BE88h case   65:*/		return 0x8226BE8C;
		  /* 8226BE8Ch */ case   66:  		/* bl -8924 */
		/* 8226BE8Ch case   66:*/		regs.LR = 0x8226BE90; return 0x82269BB0;
		/* 8226BE8Ch case   66:*/		return 0x8226BE90;
		  /* 8226BE90h */ case   67:  		/* lis R11, -32255 */
		/* 8226BE90h case   67:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BE90h case   67:*/		return 0x8226BE94;
		  /* 8226BE94h */ case   68:  		/* mr R3, R31 */
		/* 8226BE94h case   68:*/		regs.R3 = regs.R31;
		/* 8226BE94h case   68:*/		return 0x8226BE98;
		  /* 8226BE98h */ case   69:  		/* addi R4, R11, -15864 */
		/* 8226BE98h case   69:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226BE98h case   69:*/		return 0x8226BE9C;
		  /* 8226BE9Ch */ case   70:  		/* bl -9404 */
		/* 8226BE9Ch case   70:*/		regs.LR = 0x8226BEA0; return 0x822699E0;
		/* 8226BE9Ch case   70:*/		return 0x8226BEA0;
		  /* 8226BEA0h */ case   71:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226BEA0h case   71:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226BEA0h case   71:*/		return 0x8226BEA4;
		  /* 8226BEA4h */ case   72:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226BEA4h case   72:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226BEA4h case   72:*/		return 0x8226BEA8;
		  /* 8226BEA8h */ case   73:  		/* bc 4, CR0_EQ, 20 */
		/* 8226BEA8h case   73:*/		if ( !regs.CR[0].eq ) { return 0x8226BEBC;  }
		/* 8226BEA8h case   73:*/		return 0x8226BEAC;
		  /* 8226BEACh */ case   74:  		/* lis R11, -32255 */
		/* 8226BEACh case   74:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BEACh case   74:*/		return 0x8226BEB0;
		  /* 8226BEB0h */ case   75:  		/* mr R3, R31 */
		/* 8226BEB0h case   75:*/		regs.R3 = regs.R31;
		/* 8226BEB0h case   75:*/		return 0x8226BEB4;
		  /* 8226BEB4h */ case   76:  		/* addi R4, R11, -16488 */
		/* 8226BEB4h case   76:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFBF98);
		/* 8226BEB4h case   76:*/		return 0x8226BEB8;
		  /* 8226BEB8h */ case   77:  		/* bl -9432 */
		/* 8226BEB8h case   77:*/		regs.LR = 0x8226BEBC; return 0x822699E0;
		/* 8226BEB8h case   77:*/		return 0x8226BEBC;
	}
	return 0x8226BEBC;
} // Block from 8226BD84h-8226BEBCh (78 instructions)

//////////////////////////////////////////////////////
// Block at 8226BEBCh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BEBC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BEBC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BEBC);
		  /* 8226BEBCh */ case    0:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226BEBCh case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226BEBCh case    0:*/		return 0x8226BEC0;
		  /* 8226BEC0h */ case    1:  		/* lis R10, -32252 */
		/* 8226BEC0h case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8204);
		/* 8226BEC0h case    1:*/		return 0x8226BEC4;
		  /* 8226BEC4h */ case    2:  		/* mr R3, R31 */
		/* 8226BEC4h case    2:*/		regs.R3 = regs.R31;
		/* 8226BEC4h case    2:*/		return 0x8226BEC8;
		  /* 8226BEC8h */ case    3:  		/* addi R4, R10, -22336 */
		/* 8226BEC8h case    3:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R10,0xFFFFA8C0);
		/* 8226BEC8h case    3:*/		return 0x8226BECC;
		  /* 8226BECCh */ case    4:  		/* rlwinm R5, R11, 30, 24, 31 */
		/* 8226BECCh case    4:*/		cpu::op::rlwinm<0,30,24,31>(regs,&regs.R5,regs.R11);
		/* 8226BECCh case    4:*/		return 0x8226BED0;
		  /* 8226BED0h */ case    5:  		/* bl -9456 */
		/* 8226BED0h case    5:*/		regs.LR = 0x8226BED4; return 0x822699E0;
		/* 8226BED0h case    5:*/		return 0x8226BED4;
		  /* 8226BED4h */ case    6:  		/* li R5, 0 */
		/* 8226BED4h case    6:*/		cpu::op::li<0>(regs,&regs.R5,0x0);
		/* 8226BED4h case    6:*/		return 0x8226BED8;
		  /* 8226BED8h */ case    7:  		/* lwz R11, <#[R28]> */
		/* 8226BED8h case    7:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226BED8h case    7:*/		return 0x8226BEDC;
		  /* 8226BEDCh */ case    8:  		/* li R6, 1 */
		/* 8226BEDCh case    8:*/		cpu::op::li<0>(regs,&regs.R6,0x1);
		/* 8226BEDCh case    8:*/		return 0x8226BEE0;
		  /* 8226BEE0h */ case    9:  		/* rlwinm R4, R11, 17, 31, 31 */
		/* 8226BEE0h case    9:*/		cpu::op::rlwinm<0,17,31,31>(regs,&regs.R4,regs.R11);
		/* 8226BEE0h case    9:*/		return 0x8226BEE4;
		  /* 8226BEE4h */ case   10:  		/* b -180 */
		/* 8226BEE4h case   10:*/		return 0x8226BE30;
		/* 8226BEE4h case   10:*/		return 0x8226BEE8;
		  /* 8226BEE8h */ case   11:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226BEE8h case   11:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226BEE8h case   11:*/		return 0x8226BEEC;
		  /* 8226BEECh */ case   12:  		/* cmplwi CR0, R11, 0 */
		/* 8226BEECh case   12:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BEECh case   12:*/		return 0x8226BEF0;
		  /* 8226BEF0h */ case   13:  		/* bc 12, CR0_EQ, 2488 */
		/* 8226BEF0h case   13:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BEF0h case   13:*/		return 0x8226BEF4;
		  /* 8226BEF4h */ case   14:  		/* lis R11, -32254 */
		/* 8226BEF4h case   14:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226BEF4h case   14:*/		return 0x8226BEF8;
		  /* 8226BEF8h */ case   15:  		/* mr R3, R31 */
		/* 8226BEF8h case   15:*/		regs.R3 = regs.R31;
		/* 8226BEF8h case   15:*/		return 0x8226BEFC;
		  /* 8226BEFCh */ case   16:  		/* addi R4, R11, -30880 */
		/* 8226BEFCh case   16:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226BEFCh case   16:*/		return 0x8226BF00;
		  /* 8226BF00h */ case   17:  		/* bl -9504 */
		/* 8226BF00h case   17:*/		regs.LR = 0x8226BF04; return 0x822699E0;
		/* 8226BF00h case   17:*/		return 0x8226BF04;
		  /* 8226BF04h */ case   18:  		/* lis R11, -32255 */
		/* 8226BF04h case   18:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BF04h case   18:*/		return 0x8226BF08;
		  /* 8226BF08h */ case   19:  		/* addi R4, R11, 12912 */
		/* 8226BF08h case   19:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3270);
		/* 8226BF08h case   19:*/		return 0x8226BF0C;
		  /* 8226BF0Ch */ case   20:  		/* b -132 */
		/* 8226BF0Ch case   20:*/		return 0x8226BE88;
		/* 8226BF0Ch case   20:*/		return 0x8226BF10;
		  /* 8226BF10h */ case   21:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226BF10h case   21:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226BF10h case   21:*/		return 0x8226BF14;
		  /* 8226BF14h */ case   22:  		/* cmplwi CR0, R11, 0 */
		/* 8226BF14h case   22:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BF14h case   22:*/		return 0x8226BF18;
		  /* 8226BF18h */ case   23:  		/* bc 12, CR0_EQ, 2448 */
		/* 8226BF18h case   23:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BF18h case   23:*/		return 0x8226BF1C;
		  /* 8226BF1Ch */ case   24:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226BF1Ch case   24:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226BF1Ch case   24:*/		return 0x8226BF20;
		  /* 8226BF20h */ case   25:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226BF20h case   25:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226BF20h case   25:*/		return 0x8226BF24;
		  /* 8226BF24h */ case   26:  		/* bc 12, CR0_EQ, 16 */
		/* 8226BF24h case   26:*/		if ( regs.CR[0].eq ) { return 0x8226BF34;  }
		/* 8226BF24h case   26:*/		return 0x8226BF28;
		  /* 8226BF28h */ case   27:  		/* lis R11, -32251 */
		/* 8226BF28h case   27:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BF28h case   27:*/		return 0x8226BF2C;
		  /* 8226BF2Ch */ case   28:  		/* addi R4, R11, -16344 */
		/* 8226BF2Ch case   28:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC028);
		/* 8226BF2Ch case   28:*/		return 0x8226BF30;
		  /* 8226BF30h */ case   29:  		/* b -300 */
		/* 8226BF30h case   29:*/		return 0x8226BE04;
		/* 8226BF30h case   29:*/		return 0x8226BF34;
	}
	return 0x8226BF34;
} // Block from 8226BEBCh-8226BF34h (30 instructions)

//////////////////////////////////////////////////////
// Block at 8226BF34h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BF34( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BF34) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BF34);
		  /* 8226BF34h */ case    0:  		/* lis R11, -32251 */
		/* 8226BF34h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BF34h case    0:*/		return 0x8226BF38;
		  /* 8226BF38h */ case    1:  		/* addi R4, R11, -16352 */
		/* 8226BF38h case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC020);
		/* 8226BF38h case    1:*/		return 0x8226BF3C;
		  /* 8226BF3Ch */ case    2:  		/* b -312 */
		/* 8226BF3Ch case    2:*/		return 0x8226BE04;
		/* 8226BF3Ch case    2:*/		return 0x8226BF40;
		  /* 8226BF40h */ case    3:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226BF40h case    3:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226BF40h case    3:*/		return 0x8226BF44;
		  /* 8226BF44h */ case    4:  		/* cmplwi CR0, R11, 0 */
		/* 8226BF44h case    4:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BF44h case    4:*/		return 0x8226BF48;
		  /* 8226BF48h */ case    5:  		/* bc 12, CR0_EQ, 2400 */
		/* 8226BF48h case    5:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BF48h case    5:*/		return 0x8226BF4C;
		  /* 8226BF4Ch */ case    6:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226BF4Ch case    6:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226BF4Ch case    6:*/		return 0x8226BF50;
		  /* 8226BF50h */ case    7:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226BF50h case    7:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226BF50h case    7:*/		return 0x8226BF54;
		  /* 8226BF54h */ case    8:  		/* bc 12, CR0_EQ, 16 */
		/* 8226BF54h case    8:*/		if ( regs.CR[0].eq ) { return 0x8226BF64;  }
		/* 8226BF54h case    8:*/		return 0x8226BF58;
		  /* 8226BF58h */ case    9:  		/* lis R11, -32251 */
		/* 8226BF58h case    9:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BF58h case    9:*/		return 0x8226BF5C;
		  /* 8226BF5Ch */ case   10:  		/* addi R4, R11, -16344 */
		/* 8226BF5Ch case   10:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC028);
		/* 8226BF5Ch case   10:*/		return 0x8226BF60;
		  /* 8226BF60h */ case   11:  		/* b -272 */
		/* 8226BF60h case   11:*/		return 0x8226BE50;
		/* 8226BF60h case   11:*/		return 0x8226BF64;
	}
	return 0x8226BF64;
} // Block from 8226BF34h-8226BF64h (12 instructions)

//////////////////////////////////////////////////////
// Block at 8226BF64h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BF64( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BF64) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BF64);
		  /* 8226BF64h */ case    0:  		/* lis R11, -32251 */
		/* 8226BF64h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BF64h case    0:*/		return 0x8226BF68;
		  /* 8226BF68h */ case    1:  		/* addi R4, R11, -16352 */
		/* 8226BF68h case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC020);
		/* 8226BF68h case    1:*/		return 0x8226BF6C;
		  /* 8226BF6Ch */ case    2:  		/* b -284 */
		/* 8226BF6Ch case    2:*/		return 0x8226BE50;
		/* 8226BF6Ch case    2:*/		return 0x8226BF70;
		  /* 8226BF70h */ case    3:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226BF70h case    3:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226BF70h case    3:*/		return 0x8226BF74;
		  /* 8226BF74h */ case    4:  		/* cmplwi CR0, R11, 0 */
		/* 8226BF74h case    4:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226BF74h case    4:*/		return 0x8226BF78;
		  /* 8226BF78h */ case    5:  		/* bc 12, CR0_EQ, 2352 */
		/* 8226BF78h case    5:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226BF78h case    5:*/		return 0x8226BF7C;
		  /* 8226BF7Ch */ case    6:  		/* lis R11, -32254 */
		/* 8226BF7Ch case    6:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226BF7Ch case    6:*/		return 0x8226BF80;
		  /* 8226BF80h */ case    7:  		/* mr R3, R31 */
		/* 8226BF80h case    7:*/		regs.R3 = regs.R31;
		/* 8226BF80h case    7:*/		return 0x8226BF84;
		  /* 8226BF84h */ case    8:  		/* addi R4, R11, -30880 */
		/* 8226BF84h case    8:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226BF84h case    8:*/		return 0x8226BF88;
		  /* 8226BF88h */ case    9:  		/* bl -9640 */
		/* 8226BF88h case    9:*/		regs.LR = 0x8226BF8C; return 0x822699E0;
		/* 8226BF88h case    9:*/		return 0x8226BF8C;
		  /* 8226BF8Ch */ case   10:  		/* lis R11, -32255 */
		/* 8226BF8Ch case   10:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BF8Ch case   10:*/		return 0x8226BF90;
		  /* 8226BF90h */ case   11:  		/* mr R3, R31 */
		/* 8226BF90h case   11:*/		regs.R3 = regs.R31;
		/* 8226BF90h case   11:*/		return 0x8226BF94;
		  /* 8226BF94h */ case   12:  		/* addi R4, R11, -8292 */
		/* 8226BF94h case   12:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFDF9C);
		/* 8226BF94h case   12:*/		return 0x8226BF98;
		  /* 8226BF98h */ case   13:  		/* bl -9192 */
		/* 8226BF98h case   13:*/		regs.LR = 0x8226BF9C; return 0x82269BB0;
		/* 8226BF98h case   13:*/		return 0x8226BF9C;
		  /* 8226BF9Ch */ case   14:  		/* lis R11, -32255 */
		/* 8226BF9Ch case   14:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226BF9Ch case   14:*/		return 0x8226BFA0;
		  /* 8226BFA0h */ case   15:  		/* mr R3, R31 */
		/* 8226BFA0h case   15:*/		regs.R3 = regs.R31;
		/* 8226BFA0h case   15:*/		return 0x8226BFA4;
		  /* 8226BFA4h */ case   16:  		/* addi R4, R11, -15864 */
		/* 8226BFA4h case   16:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226BFA4h case   16:*/		return 0x8226BFA8;
		  /* 8226BFA8h */ case   17:  		/* bl -9672 */
		/* 8226BFA8h case   17:*/		regs.LR = 0x8226BFAC; return 0x822699E0;
		/* 8226BFA8h case   17:*/		return 0x8226BFAC;
		  /* 8226BFACh */ case   18:  		/* lis R11, -32251 */
		/* 8226BFACh case   18:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BFACh case   18:*/		return 0x8226BFB0;
		  /* 8226BFB0h */ case   19:  		/* mr R3, R31 */
		/* 8226BFB0h case   19:*/		regs.R3 = regs.R31;
		/* 8226BFB0h case   19:*/		return 0x8226BFB4;
		  /* 8226BFB4h */ case   20:  		/* addi R4, R11, -15836 */
		/* 8226BFB4h case   20:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC224);
		/* 8226BFB4h case   20:*/		return 0x8226BFB8;
		  /* 8226BFB8h */ case   21:  		/* lhz R11, <#[R28]> */
		/* 8226BFB8h case   21:*/		cpu::mem::load16z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226BFB8h case   21:*/		return 0x8226BFBC;
		  /* 8226BFBCh */ case   22:  		/* rlwinm R5, R11, 0, 27, 31 */
		/* 8226BFBCh case   22:*/		cpu::op::rlwinm<0,0,27,31>(regs,&regs.R5,regs.R11);
		/* 8226BFBCh case   22:*/		return 0x8226BFC0;
		  /* 8226BFC0h */ case   23:  		/* bl -9696 */
		/* 8226BFC0h case   23:*/		regs.LR = 0x8226BFC4; return 0x822699E0;
		/* 8226BFC0h case   23:*/		return 0x8226BFC4;
		  /* 8226BFC4h */ case   24:  		/* lis R11, -32251 */
		/* 8226BFC4h case   24:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226BFC4h case   24:*/		return 0x8226BFC8;
		  /* 8226BFC8h */ case   25:  		/* mr R3, R31 */
		/* 8226BFC8h case   25:*/		regs.R3 = regs.R31;
		/* 8226BFC8h case   25:*/		return 0x8226BFCC;
		  /* 8226BFCCh */ case   26:  		/* addi R4, R11, -16268 */
		/* 8226BFCCh case   26:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC074);
		/* 8226BFCCh case   26:*/		return 0x8226BFD0;
		  /* 8226BFD0h */ case   27:  		/* lwz R11, <#[R28]> */
		/* 8226BFD0h case   27:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226BFD0h case   27:*/		return 0x8226BFD4;
		  /* 8226BFD4h */ case   28:  		/* rlwinm R5, R11, 0, 19, 31 */
		/* 8226BFD4h case   28:*/		cpu::op::rlwinm<0,0,19,31>(regs,&regs.R5,regs.R11);
		/* 8226BFD4h case   28:*/		return 0x8226BFD8;
		  /* 8226BFD8h */ case   29:  		/* bl -9720 */
		/* 8226BFD8h case   29:*/		regs.LR = 0x8226BFDC; return 0x822699E0;
		/* 8226BFD8h case   29:*/		return 0x8226BFDC;
		  /* 8226BFDCh */ case   30:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226BFDCh case   30:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226BFDCh case   30:*/		return 0x8226BFE0;
		  /* 8226BFE0h */ case   31:  		/* lwz R10, <#[R28]> */
		/* 8226BFE0h case   31:*/		cpu::mem::load32z( regs, &regs.R10, (uint32)(regs.R28 + 0x00000000) );
		/* 8226BFE0h case   31:*/		return 0x8226BFE4;
		  /* 8226BFE4h */ case   32:  		/* rlwinm. R9, R11, 26, 31, 31 */
		/* 8226BFE4h case   32:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R9,regs.R11);
		/* 8226BFE4h case   32:*/		return 0x8226BFE8;
		  /* 8226BFE8h */ case   33:  		/* rlwinm R11, R10, 19, 31, 31 */
		/* 8226BFE8h case   33:*/		cpu::op::rlwinm<0,19,31,31>(regs,&regs.R11,regs.R10);
		/* 8226BFE8h case   33:*/		return 0x8226BFEC;
		  /* 8226BFECh */ case   34:  		/* bc 4, CR0_EQ, 12 */
		/* 8226BFECh case   34:*/		if ( !regs.CR[0].eq ) { return 0x8226BFF8;  }
		/* 8226BFECh case   34:*/		return 0x8226BFF0;
		  /* 8226BFF0h */ case   35:  		/* cmplwi CR6, R11, 0 */
		/* 8226BFF0h case   35:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x00000000);
		/* 8226BFF0h case   35:*/		return 0x8226BFF4;
		  /* 8226BFF4h */ case   36:  		/* bc 12, CR6_EQ, 48 */
		/* 8226BFF4h case   36:*/		if ( regs.CR[6].eq ) { return 0x8226C024;  }
		/* 8226BFF4h case   36:*/		return 0x8226BFF8;
	}
	return 0x8226BFF8;
} // Block from 8226BF64h-8226BFF8h (37 instructions)

//////////////////////////////////////////////////////
// Block at 8226BFF8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226BFF8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226BFF8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226BFF8);
		  /* 8226BFF8h */ case    0:  		/* cmplwi CR6, R11, 0 */
		/* 8226BFF8h case    0:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x00000000);
		/* 8226BFF8h case    0:*/		return 0x8226BFFC;
		  /* 8226BFFCh */ case    1:  		/* bc 12, CR6_EQ, 16 */
		/* 8226BFFCh case    1:*/		if ( regs.CR[6].eq ) { return 0x8226C00C;  }
		/* 8226BFFCh case    1:*/		return 0x8226C000;
		  /* 8226C000h */ case    2:  		/* lis R11, -32255 */
		/* 8226C000h case    2:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C000h case    2:*/		return 0x8226C004;
		  /* 8226C004h */ case    3:  		/* addi R5, R11, 5224 */
		/* 8226C004h case    3:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x1468);
		/* 8226C004h case    3:*/		return 0x8226C008;
		  /* 8226C008h */ case    4:  		/* b 12 */
		/* 8226C008h case    4:*/		return 0x8226C014;
		/* 8226C008h case    4:*/		return 0x8226C00C;
	}
	return 0x8226C00C;
} // Block from 8226BFF8h-8226C00Ch (5 instructions)

//////////////////////////////////////////////////////
// Block at 8226C00Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C00C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C00C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C00C);
		  /* 8226C00Ch */ case    0:  		/* lis R11, -32255 */
		/* 8226C00Ch case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C00Ch case    0:*/		return 0x8226C010;
		  /* 8226C010h */ case    1:  		/* addi R5, R11, 5216 */
		/* 8226C010h case    1:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x1460);
		/* 8226C010h case    1:*/		return 0x8226C014;
	}
	return 0x8226C014;
} // Block from 8226C00Ch-8226C014h (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C014h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C014( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C014) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C014);
		  /* 8226C014h */ case    0:  		/* lis R11, -32251 */
		/* 8226C014h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C014h case    0:*/		return 0x8226C018;
		  /* 8226C018h */ case    1:  		/* mr R3, R31 */
		/* 8226C018h case    1:*/		regs.R3 = regs.R31;
		/* 8226C018h case    1:*/		return 0x8226C01C;
		  /* 8226C01Ch */ case    2:  		/* addi R4, R11, -15848 */
		/* 8226C01Ch case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC218);
		/* 8226C01Ch case    2:*/		return 0x8226C020;
		  /* 8226C020h */ case    3:  		/* bl -9792 */
		/* 8226C020h case    3:*/		regs.LR = 0x8226C024; return 0x822699E0;
		/* 8226C020h case    3:*/		return 0x8226C024;
	}
	return 0x8226C024;
} // Block from 8226C014h-8226C024h (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226C024h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C024( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C024) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C024);
		  /* 8226C024h */ case    0:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C024h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C024h case    0:*/		return 0x8226C028;
		  /* 8226C028h */ case    1:  		/* rlwinm. R5, R11, 21, 31, 31 */
		/* 8226C028h case    1:*/		cpu::op::rlwinm<1,21,31,31>(regs,&regs.R5,regs.R11);
		/* 8226C028h case    1:*/		return 0x8226C02C;
		  /* 8226C02Ch */ case    2:  		/* bc 12, CR0_EQ, 2172 */
		/* 8226C02Ch case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C02Ch case    2:*/		return 0x8226C030;
		  /* 8226C030h */ case    3:  		/* lis R11, -32252 */
		/* 8226C030h case    3:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8204);
		/* 8226C030h case    3:*/		return 0x8226C034;
		  /* 8226C034h */ case    4:  		/* addi R4, R11, -22516 */
		/* 8226C034h case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFA80C);
		/* 8226C034h case    4:*/		return 0x8226C038;
		  /* 8226C038h */ case    5:  		/* b 1296 */
		/* 8226C038h case    5:*/		return 0x8226C548;
		/* 8226C038h case    5:*/		return 0x8226C03C;
		  /* 8226C03Ch */ case    6:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C03Ch case    6:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C03Ch case    6:*/		return 0x8226C040;
		  /* 8226C040h */ case    7:  		/* cmplwi CR0, R11, 0 */
		/* 8226C040h case    7:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C040h case    7:*/		return 0x8226C044;
		  /* 8226C044h */ case    8:  		/* bc 12, CR0_EQ, 2148 */
		/* 8226C044h case    8:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C044h case    8:*/		return 0x8226C048;
		  /* 8226C048h */ case    9:  		/* lwz R11, <#[R28]> */
		/* 8226C048h case    9:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C048h case    9:*/		return 0x8226C04C;
		  /* 8226C04Ch */ case   10:  		/* rlwinm. R11, R11, 0, 10, 10 */
		/* 8226C04Ch case   10:*/		cpu::op::rlwinm<1,0,10,10>(regs,&regs.R11,regs.R11);
		/* 8226C04Ch case   10:*/		return 0x8226C050;
		  /* 8226C050h */ case   11:  		/* bc 12, CR0_EQ, 40 */
		/* 8226C050h case   11:*/		if ( regs.CR[0].eq ) { return 0x8226C078;  }
		/* 8226C050h case   11:*/		return 0x8226C054;
		  /* 8226C054h */ case   12:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C054h case   12:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C054h case   12:*/		return 0x8226C058;
		  /* 8226C058h */ case   13:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226C058h case   13:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226C058h case   13:*/		return 0x8226C05C;
		  /* 8226C05Ch */ case   14:  		/* bc 12, CR0_EQ, 16 */
		/* 8226C05Ch case   14:*/		if ( regs.CR[0].eq ) { return 0x8226C06C;  }
		/* 8226C05Ch case   14:*/		return 0x8226C060;
		  /* 8226C060h */ case   15:  		/* lis R11, -32251 */
		/* 8226C060h case   15:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C060h case   15:*/		return 0x8226C064;
		  /* 8226C064h */ case   16:  		/* addi R4, R11, -16344 */
		/* 8226C064h case   16:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC028);
		/* 8226C064h case   16:*/		return 0x8226C068;
		  /* 8226C068h */ case   17:  		/* b 24 */
		/* 8226C068h case   17:*/		return 0x8226C080;
		/* 8226C068h case   17:*/		return 0x8226C06C;
	}
	return 0x8226C06C;
} // Block from 8226C024h-8226C06Ch (18 instructions)

//////////////////////////////////////////////////////
// Block at 8226C06Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C06C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C06C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C06C);
		  /* 8226C06Ch */ case    0:  		/* lis R11, -32251 */
		/* 8226C06Ch case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C06Ch case    0:*/		return 0x8226C070;
		  /* 8226C070h */ case    1:  		/* addi R4, R11, -16352 */
		/* 8226C070h case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC020);
		/* 8226C070h case    1:*/		return 0x8226C074;
		  /* 8226C074h */ case    2:  		/* b 12 */
		/* 8226C074h case    2:*/		return 0x8226C080;
		/* 8226C074h case    2:*/		return 0x8226C078;
	}
	return 0x8226C078;
} // Block from 8226C06Ch-8226C078h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C078h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C078( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C078) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C078);
		  /* 8226C078h */ case    0:  		/* lis R11, -32254 */
		/* 8226C078h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C078h case    0:*/		return 0x8226C07C;
		  /* 8226C07Ch */ case    1:  		/* addi R4, R11, -30880 */
		/* 8226C07Ch case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C07Ch case    1:*/		return 0x8226C080;
	}
	return 0x8226C080;
} // Block from 8226C078h-8226C080h (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C080h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C080( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C080) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C080);
		  /* 8226C080h */ case    0:  		/* mr R3, R31 */
		/* 8226C080h case    0:*/		regs.R3 = regs.R31;
		/* 8226C080h case    0:*/		return 0x8226C084;
		  /* 8226C084h */ case    1:  		/* bl -9892 */
		/* 8226C084h case    1:*/		regs.LR = 0x8226C088; return 0x822699E0;
		/* 8226C084h case    1:*/		return 0x8226C088;
		  /* 8226C088h */ case    2:  		/* lis R11, -32255 */
		/* 8226C088h case    2:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C088h case    2:*/		return 0x8226C08C;
		  /* 8226C08Ch */ case    3:  		/* mr R3, R31 */
		/* 8226C08Ch case    3:*/		regs.R3 = regs.R31;
		/* 8226C08Ch case    3:*/		return 0x8226C090;
		  /* 8226C090h */ case    4:  		/* addi R4, R11, -8304 */
		/* 8226C090h case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFDF90);
		/* 8226C090h case    4:*/		return 0x8226C094;
		  /* 8226C094h */ case    5:  		/* bl -9444 */
		/* 8226C094h case    5:*/		regs.LR = 0x8226C098; return 0x82269BB0;
		/* 8226C094h case    5:*/		return 0x8226C098;
		  /* 8226C098h */ case    6:  		/* lis R11, -32255 */
		/* 8226C098h case    6:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C098h case    6:*/		return 0x8226C09C;
		  /* 8226C09Ch */ case    7:  		/* mr R3, R31 */
		/* 8226C09Ch case    7:*/		regs.R3 = regs.R31;
		/* 8226C09Ch case    7:*/		return 0x8226C0A0;
		  /* 8226C0A0h */ case    8:  		/* addi R4, R11, -15864 */
		/* 8226C0A0h case    8:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226C0A0h case    8:*/		return 0x8226C0A4;
		  /* 8226C0A4h */ case    9:  		/* bl -9924 */
		/* 8226C0A4h case    9:*/		regs.LR = 0x8226C0A8; return 0x822699E0;
		/* 8226C0A4h case    9:*/		return 0x8226C0A8;
		  /* 8226C0A8h */ case   10:  		/* lis R11, -32251 */
		/* 8226C0A8h case   10:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C0A8h case   10:*/		return 0x8226C0AC;
		  /* 8226C0ACh */ case   11:  		/* mr R3, R31 */
		/* 8226C0ACh case   11:*/		regs.R3 = regs.R31;
		/* 8226C0ACh case   11:*/		return 0x8226C0B0;
		  /* 8226C0B0h */ case   12:  		/* addi R4, R11, -15836 */
		/* 8226C0B0h case   12:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC224);
		/* 8226C0B0h case   12:*/		return 0x8226C0B4;
		  /* 8226C0B4h */ case   13:  		/* lhz R11, <#[R28]> */
		/* 8226C0B4h case   13:*/		cpu::mem::load16z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C0B4h case   13:*/		return 0x8226C0B8;
		  /* 8226C0B8h */ case   14:  		/* rlwinm R5, R11, 0, 27, 31 */
		/* 8226C0B8h case   14:*/		cpu::op::rlwinm<0,0,27,31>(regs,&regs.R5,regs.R11);
		/* 8226C0B8h case   14:*/		return 0x8226C0BC;
		  /* 8226C0BCh */ case   15:  		/* bl -9948 */
		/* 8226C0BCh case   15:*/		regs.LR = 0x8226C0C0; return 0x822699E0;
		/* 8226C0BCh case   15:*/		return 0x8226C0C0;
		  /* 8226C0C0h */ case   16:  		/* lis R11, -32251 */
		/* 8226C0C0h case   16:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C0C0h case   16:*/		return 0x8226C0C4;
		  /* 8226C0C4h */ case   17:  		/* mr R3, R31 */
		/* 8226C0C4h case   17:*/		regs.R3 = regs.R31;
		/* 8226C0C4h case   17:*/		return 0x8226C0C8;
		  /* 8226C0C8h */ case   18:  		/* addi R4, R11, -16268 */
		/* 8226C0C8h case   18:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC074);
		/* 8226C0C8h case   18:*/		return 0x8226C0CC;
		  /* 8226C0CCh */ case   19:  		/* lwz R11, <#[R28]> */
		/* 8226C0CCh case   19:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C0CCh case   19:*/		return 0x8226C0D0;
		  /* 8226C0D0h */ case   20:  		/* rlwinm R5, R11, 0, 19, 31 */
		/* 8226C0D0h case   20:*/		cpu::op::rlwinm<0,0,19,31>(regs,&regs.R5,regs.R11);
		/* 8226C0D0h case   20:*/		return 0x8226C0D4;
		  /* 8226C0D4h */ case   21:  		/* bl -9972 */
		/* 8226C0D4h case   21:*/		regs.LR = 0x8226C0D8; return 0x822699E0;
		/* 8226C0D4h case   21:*/		return 0x8226C0D8;
		  /* 8226C0D8h */ case   22:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C0D8h case   22:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C0D8h case   22:*/		return 0x8226C0DC;
		  /* 8226C0DCh */ case   23:  		/* rlwinm. R11, R11, 0, 20, 20 */
		/* 8226C0DCh case   23:*/		cpu::op::rlwinm<1,0,20,20>(regs,&regs.R11,regs.R11);
		/* 8226C0DCh case   23:*/		return 0x8226C0E0;
		  /* 8226C0E0h */ case   24:  		/* bc 12, CR0_EQ, 1992 */
		/* 8226C0E0h case   24:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C0E0h case   24:*/		return 0x8226C0E4;
		  /* 8226C0E4h */ case   25:  		/* lis R11, -32251 */
		/* 8226C0E4h case   25:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C0E4h case   25:*/		return 0x8226C0E8;
		  /* 8226C0E8h */ case   26:  		/* addi R4, R11, -15868 */
		/* 8226C0E8h case   26:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC204);
		/* 8226C0E8h case   26:*/		return 0x8226C0EC;
		  /* 8226C0ECh */ case   27:  		/* mr R3, R31 */
		/* 8226C0ECh case   27:*/		regs.R3 = regs.R31;
		/* 8226C0ECh case   27:*/		return 0x8226C0F0;
		  /* 8226C0F0h */ case   28:  		/* bl -10000 */
		/* 8226C0F0h case   28:*/		regs.LR = 0x8226C0F4; return 0x822699E0;
		/* 8226C0F0h case   28:*/		return 0x8226C0F4;
		  /* 8226C0F4h */ case   29:  		/* b 1972 */
		/* 8226C0F4h case   29:*/		return 0x8226C8A8;
		/* 8226C0F4h case   29:*/		return 0x8226C0F8;
		  /* 8226C0F8h */ case   30:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C0F8h case   30:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C0F8h case   30:*/		return 0x8226C0FC;
		  /* 8226C0FCh */ case   31:  		/* cmplwi CR0, R11, 0 */
		/* 8226C0FCh case   31:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C0FCh case   31:*/		return 0x8226C100;
		  /* 8226C100h */ case   32:  		/* bc 12, CR0_EQ, 1960 */
		/* 8226C100h case   32:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C100h case   32:*/		return 0x8226C104;
		  /* 8226C104h */ case   33:  		/* lwz R11, <#[R28]> */
		/* 8226C104h case   33:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C104h case   33:*/		return 0x8226C108;
		  /* 8226C108h */ case   34:  		/* li R30, 0 */
		/* 8226C108h case   34:*/		cpu::op::li<0>(regs,&regs.R30,0x0);
		/* 8226C108h case   34:*/		return 0x8226C10C;
		  /* 8226C10Ch */ case   35:  		/* rlwinm. R11, R11, 0, 17, 17 */
		/* 8226C10Ch case   35:*/		cpu::op::rlwinm<1,0,17,17>(regs,&regs.R11,regs.R11);
		/* 8226C10Ch case   35:*/		return 0x8226C110;
		  /* 8226C110h */ case   36:  		/* bc 12, CR0_EQ, 44 */
		/* 8226C110h case   36:*/		if ( regs.CR[0].eq ) { return 0x8226C13C;  }
		/* 8226C110h case   36:*/		return 0x8226C114;
		  /* 8226C114h */ case   37:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C114h case   37:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C114h case   37:*/		return 0x8226C118;
		  /* 8226C118h */ case   38:  		/* li R30, 1 */
		/* 8226C118h case   38:*/		cpu::op::li<0>(regs,&regs.R30,0x1);
		/* 8226C118h case   38:*/		return 0x8226C11C;
		  /* 8226C11Ch */ case   39:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226C11Ch case   39:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226C11Ch case   39:*/		return 0x8226C120;
		  /* 8226C120h */ case   40:  		/* bc 12, CR0_EQ, 16 */
		/* 8226C120h case   40:*/		if ( regs.CR[0].eq ) { return 0x8226C130;  }
		/* 8226C120h case   40:*/		return 0x8226C124;
		  /* 8226C124h */ case   41:  		/* lis R11, -32251 */
		/* 8226C124h case   41:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C124h case   41:*/		return 0x8226C128;
		  /* 8226C128h */ case   42:  		/* addi R4, R11, -16344 */
		/* 8226C128h case   42:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC028);
		/* 8226C128h case   42:*/		return 0x8226C12C;
		  /* 8226C12Ch */ case   43:  		/* b 24 */
		/* 8226C12Ch case   43:*/		return 0x8226C144;
		/* 8226C12Ch case   43:*/		return 0x8226C130;
	}
	return 0x8226C130;
} // Block from 8226C080h-8226C130h (44 instructions)

//////////////////////////////////////////////////////
// Block at 8226C130h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C130( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C130) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C130);
		  /* 8226C130h */ case    0:  		/* lis R11, -32251 */
		/* 8226C130h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C130h case    0:*/		return 0x8226C134;
		  /* 8226C134h */ case    1:  		/* addi R4, R11, -16352 */
		/* 8226C134h case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC020);
		/* 8226C134h case    1:*/		return 0x8226C138;
		  /* 8226C138h */ case    2:  		/* b 12 */
		/* 8226C138h case    2:*/		return 0x8226C144;
		/* 8226C138h case    2:*/		return 0x8226C13C;
	}
	return 0x8226C13C;
} // Block from 8226C130h-8226C13Ch (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C13Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C13C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C13C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C13C);
		  /* 8226C13Ch */ case    0:  		/* lis R11, -32254 */
		/* 8226C13Ch case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C13Ch case    0:*/		return 0x8226C140;
		  /* 8226C140h */ case    1:  		/* addi R4, R11, -30880 */
		/* 8226C140h case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C140h case    1:*/		return 0x8226C144;
	}
	return 0x8226C144;
} // Block from 8226C13Ch-8226C144h (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C144h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C144( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C144) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C144);
		  /* 8226C144h */ case    0:  		/* mr R3, R31 */
		/* 8226C144h case    0:*/		regs.R3 = regs.R31;
		/* 8226C144h case    0:*/		return 0x8226C148;
		  /* 8226C148h */ case    1:  		/* bl -10088 */
		/* 8226C148h case    1:*/		regs.LR = 0x8226C14C; return 0x822699E0;
		/* 8226C148h case    1:*/		return 0x8226C14C;
		  /* 8226C14Ch */ case    2:  		/* lwz R11, <#[R28]> */
		/* 8226C14Ch case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C14Ch case    2:*/		return 0x8226C150;
		  /* 8226C150h */ case    3:  		/* rlwinm. R11, R11, 0, 18, 18 */
		/* 8226C150h case    3:*/		cpu::op::rlwinm<1,0,18,18>(regs,&regs.R11,regs.R11);
		/* 8226C150h case    3:*/		return 0x8226C154;
		  /* 8226C154h */ case    4:  		/* bc 4, CR0_EQ, 120 */
		/* 8226C154h case    4:*/		if ( !regs.CR[0].eq ) { return 0x8226C1CC;  }
		/* 8226C154h case    4:*/		return 0x8226C158;
		  /* 8226C158h */ case    5:  		/* rlwinm. R11, R30, 0, 24, 31 */
		/* 8226C158h case    5:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R11,regs.R30);
		/* 8226C158h case    5:*/		return 0x8226C15C;
		  /* 8226C15Ch */ case    6:  		/* bc 4, CR0_EQ, 112 */
		/* 8226C15Ch case    6:*/		if ( !regs.CR[0].eq ) { return 0x8226C1CC;  }
		/* 8226C15Ch case    6:*/		return 0x8226C160;
		  /* 8226C160h */ case    7:  		/* lis R11, -32255 */
		/* 8226C160h case    7:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C160h case    7:*/		return 0x8226C164;
		  /* 8226C164h */ case    8:  		/* mr R3, R31 */
		/* 8226C164h case    8:*/		regs.R3 = regs.R31;
		/* 8226C164h case    8:*/		return 0x8226C168;
		  /* 8226C168h */ case    9:  		/* addi R4, R11, 12904 */
		/* 8226C168h case    9:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3268);
		/* 8226C168h case    9:*/		return 0x8226C16C;
		  /* 8226C16Ch */ case   10:  		/* bl -9660 */
		/* 8226C16Ch case   10:*/		regs.LR = 0x8226C170; return 0x82269BB0;
		/* 8226C16Ch case   10:*/		return 0x8226C170;
		  /* 8226C170h */ case   11:  		/* lis R11, -32255 */
		/* 8226C170h case   11:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C170h case   11:*/		return 0x8226C174;
		  /* 8226C174h */ case   12:  		/* mr R3, R31 */
		/* 8226C174h case   12:*/		regs.R3 = regs.R31;
		/* 8226C174h case   12:*/		return 0x8226C178;
		  /* 8226C178h */ case   13:  		/* addi R4, R11, -15864 */
		/* 8226C178h case   13:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226C178h case   13:*/		return 0x8226C17C;
		  /* 8226C17Ch */ case   14:  		/* bl -10140 */
		/* 8226C17Ch case   14:*/		regs.LR = 0x8226C180; return 0x822699E0;
		/* 8226C17Ch case   14:*/		return 0x8226C180;
		  /* 8226C180h */ case   15:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C180h case   15:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C180h case   15:*/		return 0x8226C184;
		  /* 8226C184h */ case   16:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226C184h case   16:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226C184h case   16:*/		return 0x8226C188;
		  /* 8226C188h */ case   17:  		/* bc 12, CR0_EQ, 16 */
		/* 8226C188h case   17:*/		if ( regs.CR[0].eq ) { return 0x8226C198;  }
		/* 8226C188h case   17:*/		return 0x8226C18C;
		  /* 8226C18Ch */ case   18:  		/* lis R11, -32256 */
		/* 8226C18Ch case   18:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8200);
		/* 8226C18Ch case   18:*/		return 0x8226C190;
		  /* 8226C190h */ case   19:  		/* addi R5, R11, 4485 */
		/* 8226C190h case   19:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x1185);
		/* 8226C190h case   19:*/		return 0x8226C194;
		  /* 8226C194h */ case   20:  		/* b 12 */
		/* 8226C194h case   20:*/		return 0x8226C1A0;
		/* 8226C194h case   20:*/		return 0x8226C198;
	}
	return 0x8226C198;
} // Block from 8226C144h-8226C198h (21 instructions)

//////////////////////////////////////////////////////
// Block at 8226C198h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C198( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C198) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C198);
		  /* 8226C198h */ case    0:  		/* lis R11, -32255 */
		/* 8226C198h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C198h case    0:*/		return 0x8226C19C;
		  /* 8226C19Ch */ case    1:  		/* addi R5, R11, -16488 */
		/* 8226C19Ch case    1:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0xFFFFBF98);
		/* 8226C19Ch case    1:*/		return 0x8226C1A0;
	}
	return 0x8226C1A0;
} // Block from 8226C198h-8226C1A0h (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C1A0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C1A0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C1A0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C1A0);
		  /* 8226C1A0h */ case    0:  		/* lis R11, -32255 */
		/* 8226C1A0h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C1A0h case    0:*/		return 0x8226C1A4;
		  /* 8226C1A4h */ case    1:  		/* mr R3, R31 */
		/* 8226C1A4h case    1:*/		regs.R3 = regs.R31;
		/* 8226C1A4h case    1:*/		return 0x8226C1A8;
		  /* 8226C1A8h */ case    2:  		/* addi R4, R11, -10432 */
		/* 8226C1A8h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFD740);
		/* 8226C1A8h case    2:*/		return 0x8226C1AC;
		  /* 8226C1ACh */ case    3:  		/* bl -10188 */
		/* 8226C1ACh case    3:*/		regs.LR = 0x8226C1B0; return 0x822699E0;
		/* 8226C1ACh case    3:*/		return 0x8226C1B0;
		  /* 8226C1B0h */ case    4:  		/* lis R11, -32251 */
		/* 8226C1B0h case    4:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C1B0h case    4:*/		return 0x8226C1B4;
		  /* 8226C1B4h */ case    5:  		/* mr R3, R31 */
		/* 8226C1B4h case    5:*/		regs.R3 = regs.R31;
		/* 8226C1B4h case    5:*/		return 0x8226C1B8;
		  /* 8226C1B8h */ case    6:  		/* addi R4, R11, -15876 */
		/* 8226C1B8h case    6:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC1FC);
		/* 8226C1B8h case    6:*/		return 0x8226C1BC;
		  /* 8226C1BCh */ case    7:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C1BCh case    7:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C1BCh case    7:*/		return 0x8226C1C0;
		  /* 8226C1C0h */ case    8:  		/* rlwinm R5, R11, 30, 24, 31 */
		/* 8226C1C0h case    8:*/		cpu::op::rlwinm<0,30,24,31>(regs,&regs.R5,regs.R11);
		/* 8226C1C0h case    8:*/		return 0x8226C1C4;
		  /* 8226C1C4h */ case    9:  		/* bl -10212 */
		/* 8226C1C4h case    9:*/		regs.LR = 0x8226C1C8; return 0x822699E0;
		/* 8226C1C4h case    9:*/		return 0x8226C1C8;
		  /* 8226C1C8h */ case   10:  		/* b 36 */
		/* 8226C1C8h case   10:*/		return 0x8226C1EC;
		/* 8226C1C8h case   10:*/		return 0x8226C1CC;
	}
	return 0x8226C1CC;
} // Block from 8226C1A0h-8226C1CCh (11 instructions)

//////////////////////////////////////////////////////
// Block at 8226C1CCh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C1CC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C1CC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C1CC);
		  /* 8226C1CCh */ case    0:  		/* lis R11, -32255 */
		/* 8226C1CCh case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C1CCh case    0:*/		return 0x8226C1D0;
		  /* 8226C1D0h */ case    1:  		/* mr R3, R31 */
		/* 8226C1D0h case    1:*/		regs.R3 = regs.R31;
		/* 8226C1D0h case    1:*/		return 0x8226C1D4;
		  /* 8226C1D4h */ case    2:  		/* addi R4, R11, -8276 */
		/* 8226C1D4h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFDFAC);
		/* 8226C1D4h case    2:*/		return 0x8226C1D8;
		  /* 8226C1D8h */ case    3:  		/* bl -9768 */
		/* 8226C1D8h case    3:*/		regs.LR = 0x8226C1DC; return 0x82269BB0;
		/* 8226C1D8h case    3:*/		return 0x8226C1DC;
		  /* 8226C1DCh */ case    4:  		/* lis R11, -32255 */
		/* 8226C1DCh case    4:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C1DCh case    4:*/		return 0x8226C1E0;
		  /* 8226C1E0h */ case    5:  		/* mr R3, R31 */
		/* 8226C1E0h case    5:*/		regs.R3 = regs.R31;
		/* 8226C1E0h case    5:*/		return 0x8226C1E4;
		  /* 8226C1E4h */ case    6:  		/* addi R4, R11, -15864 */
		/* 8226C1E4h case    6:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226C1E4h case    6:*/		return 0x8226C1E8;
		  /* 8226C1E8h */ case    7:  		/* bl -10248 */
		/* 8226C1E8h case    7:*/		regs.LR = 0x8226C1EC; return 0x822699E0;
		/* 8226C1E8h case    7:*/		return 0x8226C1EC;
	}
	return 0x8226C1EC;
} // Block from 8226C1CCh-8226C1ECh (8 instructions)

//////////////////////////////////////////////////////
// Block at 8226C1ECh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C1EC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C1EC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C1EC);
		  /* 8226C1ECh */ case    0:  		/* lwz R11, <#[R28]> */
		/* 8226C1ECh case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C1ECh case    0:*/		return 0x8226C1F0;
		  /* 8226C1F0h */ case    1:  		/* lis R10, -32251 */
		/* 8226C1F0h case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8205);
		/* 8226C1F0h case    1:*/		return 0x8226C1F4;
		  /* 8226C1F4h */ case    2:  		/* addi R4, R10, -16268 */
		/* 8226C1F4h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R10,0xFFFFC074);
		/* 8226C1F4h case    2:*/		return 0x8226C1F8;
		  /* 8226C1F8h */ case    3:  		/* rlwinm R5, R11, 0, 19, 31 */
		/* 8226C1F8h case    3:*/		cpu::op::rlwinm<0,0,19,31>(regs,&regs.R5,regs.R11);
		/* 8226C1F8h case    3:*/		return 0x8226C1FC;
		  /* 8226C1FCh */ case    4:  		/* b 844 */
		/* 8226C1FCh case    4:*/		return 0x8226C548;
		/* 8226C1FCh case    4:*/		return 0x8226C200;
		  /* 8226C200h */ case    5:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C200h case    5:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C200h case    5:*/		return 0x8226C204;
		  /* 8226C204h */ case    6:  		/* cmplwi CR0, R11, 0 */
		/* 8226C204h case    6:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C204h case    6:*/		return 0x8226C208;
		  /* 8226C208h */ case    7:  		/* bc 12, CR0_EQ, 1696 */
		/* 8226C208h case    7:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C208h case    7:*/		return 0x8226C20C;
		  /* 8226C20Ch */ case    8:  		/* lis R11, -32254 */
		/* 8226C20Ch case    8:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C20Ch case    8:*/		return 0x8226C210;
		  /* 8226C210h */ case    9:  		/* mr R3, R31 */
		/* 8226C210h case    9:*/		regs.R3 = regs.R31;
		/* 8226C210h case    9:*/		return 0x8226C214;
		  /* 8226C214h */ case   10:  		/* addi R4, R11, -30880 */
		/* 8226C214h case   10:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C214h case   10:*/		return 0x8226C218;
		  /* 8226C218h */ case   11:  		/* bl -10296 */
		/* 8226C218h case   11:*/		regs.LR = 0x8226C21C; return 0x822699E0;
		/* 8226C218h case   11:*/		return 0x8226C21C;
		  /* 8226C21Ch */ case   12:  		/* lis R11, -32255 */
		/* 8226C21Ch case   12:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C21Ch case   12:*/		return 0x8226C220;
		  /* 8226C220h */ case   13:  		/* addi R4, R11, -8296 */
		/* 8226C220h case   13:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFDF98);
		/* 8226C220h case   13:*/		return 0x8226C224;
		  /* 8226C224h */ case   14:  		/* b -1088 */
		/* 8226C224h case   14:*/		return 0x8226BDE4;
		/* 8226C224h case   14:*/		return 0x8226C228;
		  /* 8226C228h */ case   15:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C228h case   15:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C228h case   15:*/		return 0x8226C22C;
		  /* 8226C22Ch */ case   16:  		/* cmplwi CR0, R11, 0 */
		/* 8226C22Ch case   16:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C22Ch case   16:*/		return 0x8226C230;
		  /* 8226C230h */ case   17:  		/* bc 12, CR0_EQ, 1656 */
		/* 8226C230h case   17:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C230h case   17:*/		return 0x8226C234;
		  /* 8226C234h */ case   18:  		/* lwz R11, <#[R28]> */
		/* 8226C234h case   18:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C234h case   18:*/		return 0x8226C238;
		  /* 8226C238h */ case   19:  		/* li R30, 0 */
		/* 8226C238h case   19:*/		cpu::op::li<0>(regs,&regs.R30,0x0);
		/* 8226C238h case   19:*/		return 0x8226C23C;
		  /* 8226C23Ch */ case   20:  		/* rlwinm. R11, R11, 0, 17, 17 */
		/* 8226C23Ch case   20:*/		cpu::op::rlwinm<1,0,17,17>(regs,&regs.R11,regs.R11);
		/* 8226C23Ch case   20:*/		return 0x8226C240;
		  /* 8226C240h */ case   21:  		/* bc 12, CR0_EQ, 44 */
		/* 8226C240h case   21:*/		if ( regs.CR[0].eq ) { return 0x8226C26C;  }
		/* 8226C240h case   21:*/		return 0x8226C244;
		  /* 8226C244h */ case   22:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C244h case   22:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C244h case   22:*/		return 0x8226C248;
		  /* 8226C248h */ case   23:  		/* li R30, 1 */
		/* 8226C248h case   23:*/		cpu::op::li<0>(regs,&regs.R30,0x1);
		/* 8226C248h case   23:*/		return 0x8226C24C;
		  /* 8226C24Ch */ case   24:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226C24Ch case   24:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226C24Ch case   24:*/		return 0x8226C250;
		  /* 8226C250h */ case   25:  		/* bc 12, CR0_EQ, 16 */
		/* 8226C250h case   25:*/		if ( regs.CR[0].eq ) { return 0x8226C260;  }
		/* 8226C250h case   25:*/		return 0x8226C254;
		  /* 8226C254h */ case   26:  		/* lis R11, -32251 */
		/* 8226C254h case   26:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C254h case   26:*/		return 0x8226C258;
		  /* 8226C258h */ case   27:  		/* addi R4, R11, -16344 */
		/* 8226C258h case   27:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC028);
		/* 8226C258h case   27:*/		return 0x8226C25C;
		  /* 8226C25Ch */ case   28:  		/* b 24 */
		/* 8226C25Ch case   28:*/		return 0x8226C274;
		/* 8226C25Ch case   28:*/		return 0x8226C260;
	}
	return 0x8226C260;
} // Block from 8226C1ECh-8226C260h (29 instructions)

//////////////////////////////////////////////////////
// Block at 8226C260h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C260( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C260) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C260);
		  /* 8226C260h */ case    0:  		/* lis R11, -32251 */
		/* 8226C260h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C260h case    0:*/		return 0x8226C264;
		  /* 8226C264h */ case    1:  		/* addi R4, R11, -16352 */
		/* 8226C264h case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC020);
		/* 8226C264h case    1:*/		return 0x8226C268;
		  /* 8226C268h */ case    2:  		/* b 12 */
		/* 8226C268h case    2:*/		return 0x8226C274;
		/* 8226C268h case    2:*/		return 0x8226C26C;
	}
	return 0x8226C26C;
} // Block from 8226C260h-8226C26Ch (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C26Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C26C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C26C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C26C);
		  /* 8226C26Ch */ case    0:  		/* lis R11, -32254 */
		/* 8226C26Ch case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C26Ch case    0:*/		return 0x8226C270;
		  /* 8226C270h */ case    1:  		/* addi R4, R11, -30880 */
		/* 8226C270h case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C270h case    1:*/		return 0x8226C274;
	}
	return 0x8226C274;
} // Block from 8226C26Ch-8226C274h (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C274h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C274( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C274) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C274);
		  /* 8226C274h */ case    0:  		/* mr R3, R31 */
		/* 8226C274h case    0:*/		regs.R3 = regs.R31;
		/* 8226C274h case    0:*/		return 0x8226C278;
		  /* 8226C278h */ case    1:  		/* bl -10392 */
		/* 8226C278h case    1:*/		regs.LR = 0x8226C27C; return 0x822699E0;
		/* 8226C278h case    1:*/		return 0x8226C27C;
		  /* 8226C27Ch */ case    2:  		/* lwz R10, <#[R28]> */
		/* 8226C27Ch case    2:*/		cpu::mem::load32z( regs, &regs.R10, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C27Ch case    2:*/		return 0x8226C280;
		  /* 8226C280h */ case    3:  		/* li R11, 0 */
		/* 8226C280h case    3:*/		cpu::op::li<0>(regs,&regs.R11,0x0);
		/* 8226C280h case    3:*/		return 0x8226C284;
		  /* 8226C284h */ case    4:  		/* rlwinm. R10, R10, 0, 18, 18 */
		/* 8226C284h case    4:*/		cpu::op::rlwinm<1,0,18,18>(regs,&regs.R10,regs.R10);
		/* 8226C284h case    4:*/		return 0x8226C288;
		  /* 8226C288h */ case    5:  		/* bc 4, CR0_EQ, 12 */
		/* 8226C288h case    5:*/		if ( !regs.CR[0].eq ) { return 0x8226C294;  }
		/* 8226C288h case    5:*/		return 0x8226C28C;
		  /* 8226C28Ch */ case    6:  		/* rlwinm. R10, R30, 0, 24, 31 */
		/* 8226C28Ch case    6:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R10,regs.R30);
		/* 8226C28Ch case    6:*/		return 0x8226C290;
		  /* 8226C290h */ case    7:  		/* bc 12, CR0_EQ, 8 */
		/* 8226C290h case    7:*/		if ( regs.CR[0].eq ) { return 0x8226C298;  }
		/* 8226C290h case    7:*/		return 0x8226C294;
	}
	return 0x8226C294;
} // Block from 8226C274h-8226C294h (8 instructions)

//////////////////////////////////////////////////////
// Block at 8226C294h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C294( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C294) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C294);
		  /* 8226C294h */ case    0:  		/* li R11, 1 */
		/* 8226C294h case    0:*/		cpu::op::li<0>(regs,&regs.R11,0x1);
		/* 8226C294h case    0:*/		return 0x8226C298;
	}
	return 0x8226C298;
} // Block from 8226C294h-8226C298h (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226C298h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C298( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C298) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C298);
		  /* 8226C298h */ case    0:  		/* rlwinm. R11, R11, 0, 24, 31 */
		/* 8226C298h case    0:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R11,regs.R11);
		/* 8226C298h case    0:*/		return 0x8226C29C;
		  /* 8226C29Ch */ case    1:  		/* mr R3, R31 */
		/* 8226C29Ch case    1:*/		regs.R3 = regs.R31;
		/* 8226C29Ch case    1:*/		return 0x8226C2A0;
		  /* 8226C2A0h */ case    2:  		/* bc 12, CR0_EQ, 36 */
		/* 8226C2A0h case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C2C4;  }
		/* 8226C2A0h case    2:*/		return 0x8226C2A4;
		  /* 8226C2A4h */ case    3:  		/* lis R11, -32255 */
		/* 8226C2A4h case    3:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C2A4h case    3:*/		return 0x8226C2A8;
		  /* 8226C2A8h */ case    4:  		/* addi R4, R11, 12892 */
		/* 8226C2A8h case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x325C);
		/* 8226C2A8h case    4:*/		return 0x8226C2AC;
		  /* 8226C2ACh */ case    5:  		/* bl -9980 */
		/* 8226C2ACh case    5:*/		regs.LR = 0x8226C2B0; return 0x82269BB0;
		/* 8226C2ACh case    5:*/		return 0x8226C2B0;
		  /* 8226C2B0h */ case    6:  		/* lis R11, -32255 */
		/* 8226C2B0h case    6:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C2B0h case    6:*/		return 0x8226C2B4;
		  /* 8226C2B4h */ case    7:  		/* mr R3, R31 */
		/* 8226C2B4h case    7:*/		regs.R3 = regs.R31;
		/* 8226C2B4h case    7:*/		return 0x8226C2B8;
		  /* 8226C2B8h */ case    8:  		/* addi R4, R11, -15864 */
		/* 8226C2B8h case    8:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226C2B8h case    8:*/		return 0x8226C2BC;
		  /* 8226C2BCh */ case    9:  		/* bl -10460 */
		/* 8226C2BCh case    9:*/		regs.LR = 0x8226C2C0; return 0x822699E0;
		/* 8226C2BCh case    9:*/		return 0x8226C2C0;
		  /* 8226C2C0h */ case   10:  		/* b 104 */
		/* 8226C2C0h case   10:*/		return 0x8226C328;
		/* 8226C2C0h case   10:*/		return 0x8226C2C4;
	}
	return 0x8226C2C4;
} // Block from 8226C298h-8226C2C4h (11 instructions)

//////////////////////////////////////////////////////
// Block at 8226C2C4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C2C4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C2C4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C2C4);
		  /* 8226C2C4h */ case    0:  		/* lis R11, -32255 */
		/* 8226C2C4h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C2C4h case    0:*/		return 0x8226C2C8;
		  /* 8226C2C8h */ case    1:  		/* addi R4, R11, 12896 */
		/* 8226C2C8h case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3260);
		/* 8226C2C8h case    1:*/		return 0x8226C2CC;
		  /* 8226C2CCh */ case    2:  		/* bl -10012 */
		/* 8226C2CCh case    2:*/		regs.LR = 0x8226C2D0; return 0x82269BB0;
		/* 8226C2CCh case    2:*/		return 0x8226C2D0;
		  /* 8226C2D0h */ case    3:  		/* lis R11, -32255 */
		/* 8226C2D0h case    3:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C2D0h case    3:*/		return 0x8226C2D4;
		  /* 8226C2D4h */ case    4:  		/* mr R3, R31 */
		/* 8226C2D4h case    4:*/		regs.R3 = regs.R31;
		/* 8226C2D4h case    4:*/		return 0x8226C2D8;
		  /* 8226C2D8h */ case    5:  		/* addi R4, R11, -15864 */
		/* 8226C2D8h case    5:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226C2D8h case    5:*/		return 0x8226C2DC;
		  /* 8226C2DCh */ case    6:  		/* bl -10492 */
		/* 8226C2DCh case    6:*/		regs.LR = 0x8226C2E0; return 0x822699E0;
		/* 8226C2DCh case    6:*/		return 0x8226C2E0;
		  /* 8226C2E0h */ case    7:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C2E0h case    7:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C2E0h case    7:*/		return 0x8226C2E4;
		  /* 8226C2E4h */ case    8:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226C2E4h case    8:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226C2E4h case    8:*/		return 0x8226C2E8;
		  /* 8226C2E8h */ case    9:  		/* bc 12, CR0_EQ, 16 */
		/* 8226C2E8h case    9:*/		if ( regs.CR[0].eq ) { return 0x8226C2F8;  }
		/* 8226C2E8h case    9:*/		return 0x8226C2EC;
		  /* 8226C2ECh */ case   10:  		/* lis R11, -32256 */
		/* 8226C2ECh case   10:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8200);
		/* 8226C2ECh case   10:*/		return 0x8226C2F0;
		  /* 8226C2F0h */ case   11:  		/* addi R5, R11, 4485 */
		/* 8226C2F0h case   11:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x1185);
		/* 8226C2F0h case   11:*/		return 0x8226C2F4;
		  /* 8226C2F4h */ case   12:  		/* b 12 */
		/* 8226C2F4h case   12:*/		return 0x8226C300;
		/* 8226C2F4h case   12:*/		return 0x8226C2F8;
	}
	return 0x8226C2F8;
} // Block from 8226C2C4h-8226C2F8h (13 instructions)

//////////////////////////////////////////////////////
// Block at 8226C2F8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C2F8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C2F8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C2F8);
		  /* 8226C2F8h */ case    0:  		/* lis R11, -32255 */
		/* 8226C2F8h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C2F8h case    0:*/		return 0x8226C2FC;
		  /* 8226C2FCh */ case    1:  		/* addi R5, R11, -16488 */
		/* 8226C2FCh case    1:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0xFFFFBF98);
		/* 8226C2FCh case    1:*/		return 0x8226C300;
	}
	return 0x8226C300;
} // Block from 8226C2F8h-8226C300h (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C300h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C300( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C300) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C300);
		  /* 8226C300h */ case    0:  		/* lis R11, -32255 */
		/* 8226C300h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C300h case    0:*/		return 0x8226C304;
		  /* 8226C304h */ case    1:  		/* mr R3, R31 */
		/* 8226C304h case    1:*/		regs.R3 = regs.R31;
		/* 8226C304h case    1:*/		return 0x8226C308;
		  /* 8226C308h */ case    2:  		/* addi R4, R11, -10432 */
		/* 8226C308h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFD740);
		/* 8226C308h case    2:*/		return 0x8226C30C;
		  /* 8226C30Ch */ case    3:  		/* bl -10540 */
		/* 8226C30Ch case    3:*/		regs.LR = 0x8226C310; return 0x822699E0;
		/* 8226C30Ch case    3:*/		return 0x8226C310;
		  /* 8226C310h */ case    4:  		/* lis R11, -32251 */
		/* 8226C310h case    4:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C310h case    4:*/		return 0x8226C314;
		  /* 8226C314h */ case    5:  		/* mr R3, R31 */
		/* 8226C314h case    5:*/		regs.R3 = regs.R31;
		/* 8226C314h case    5:*/		return 0x8226C318;
		  /* 8226C318h */ case    6:  		/* addi R4, R11, -15876 */
		/* 8226C318h case    6:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC1FC);
		/* 8226C318h case    6:*/		return 0x8226C31C;
		  /* 8226C31Ch */ case    7:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C31Ch case    7:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C31Ch case    7:*/		return 0x8226C320;
		  /* 8226C320h */ case    8:  		/* rlwinm R5, R11, 30, 24, 31 */
		/* 8226C320h case    8:*/		cpu::op::rlwinm<0,30,24,31>(regs,&regs.R5,regs.R11);
		/* 8226C320h case    8:*/		return 0x8226C324;
		  /* 8226C324h */ case    9:  		/* bl -10564 */
		/* 8226C324h case    9:*/		regs.LR = 0x8226C328; return 0x822699E0;
		/* 8226C324h case    9:*/		return 0x8226C328;
	}
	return 0x8226C328;
} // Block from 8226C300h-8226C328h (10 instructions)

//////////////////////////////////////////////////////
// Block at 8226C328h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C328( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C328) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C328);
		  /* 8226C328h */ case    0:  		/* lwz R11, <#[R28]> */
		/* 8226C328h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C328h case    0:*/		return 0x8226C32C;
		  /* 8226C32Ch */ case    1:  		/* lis R10, -32251 */
		/* 8226C32Ch case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8205);
		/* 8226C32Ch case    1:*/		return 0x8226C330;
		  /* 8226C330h */ case    2:  		/* mr R3, R31 */
		/* 8226C330h case    2:*/		regs.R3 = regs.R31;
		/* 8226C330h case    2:*/		return 0x8226C334;
		  /* 8226C334h */ case    3:  		/* addi R4, R10, -16268 */
		/* 8226C334h case    3:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R10,0xFFFFC074);
		/* 8226C334h case    3:*/		return 0x8226C338;
		  /* 8226C338h */ case    4:  		/* rlwinm R5, R11, 0, 19, 31 */
		/* 8226C338h case    4:*/		cpu::op::rlwinm<0,0,19,31>(regs,&regs.R5,regs.R11);
		/* 8226C338h case    4:*/		return 0x8226C33C;
		  /* 8226C33Ch */ case    5:  		/* bl -10588 */
		/* 8226C33Ch case    5:*/		regs.LR = 0x8226C340; return 0x822699E0;
		/* 8226C33Ch case    5:*/		return 0x8226C340;
		  /* 8226C340h */ case    6:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226C340h case    6:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C340h case    6:*/		return 0x8226C344;
		  /* 8226C344h */ case    7:  		/* rlwinm. R10, R11, 0, 29, 29 */
		/* 8226C344h case    7:*/		cpu::op::rlwinm<1,0,29,29>(regs,&regs.R10,regs.R11);
		/* 8226C344h case    7:*/		return 0x8226C348;
		  /* 8226C348h */ case    8:  		/* bc 12, CR0_EQ, 48 */
		/* 8226C348h case    8:*/		if ( regs.CR[0].eq ) { return 0x8226C378;  }
		/* 8226C348h case    8:*/		return 0x8226C34C;
		  /* 8226C34Ch */ case    9:  		/* lwz R10, <#[R28 + 4]> */
		/* 8226C34Ch case    9:*/		cpu::mem::load32z( regs, &regs.R10, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C34Ch case    9:*/		return 0x8226C350;
		  /* 8226C350h */ case   10:  		/* rlwinm. R10, R10, 31, 31, 31 */
		/* 8226C350h case   10:*/		cpu::op::rlwinm<1,31,31,31>(regs,&regs.R10,regs.R10);
		/* 8226C350h case   10:*/		return 0x8226C354;
		  /* 8226C354h */ case   11:  		/* bc 4, CR0_EQ, 60 */
		/* 8226C354h case   11:*/		if ( !regs.CR[0].eq ) { return 0x8226C390;  }
		/* 8226C354h case   11:*/		return 0x8226C358;
		  /* 8226C358h */ case   12:  		/* rlwinm. R11, R11, 26, 31, 31 */
		/* 8226C358h case   12:*/		cpu::op::rlwinm<1,26,31,31>(regs,&regs.R11,regs.R11);
		/* 8226C358h case   12:*/		return 0x8226C35C;
		  /* 8226C35Ch */ case   13:  		/* bc 12, CR0_EQ, 28 */
		/* 8226C35Ch case   13:*/		if ( regs.CR[0].eq ) { return 0x8226C378;  }
		/* 8226C35Ch case   13:*/		return 0x8226C360;
		  /* 8226C360h */ case   14:  		/* lis R11, -32255 */
		/* 8226C360h case   14:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C360h case   14:*/		return 0x8226C364;
		  /* 8226C364h */ case   15:  		/* addi R5, R11, 5216 */
		/* 8226C364h case   15:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x1460);
		/* 8226C364h case   15:*/		return 0x8226C368;
		  /* 8226C368h */ case   16:  		/* lis R11, -32251 */
		/* 8226C368h case   16:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C368h case   16:*/		return 0x8226C36C;
		  /* 8226C36Ch */ case   17:  		/* mr R3, R31 */
		/* 8226C36Ch case   17:*/		regs.R3 = regs.R31;
		/* 8226C36Ch case   17:*/		return 0x8226C370;
		  /* 8226C370h */ case   18:  		/* addi R4, R11, -15896 */
		/* 8226C370h case   18:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC1E8);
		/* 8226C370h case   18:*/		return 0x8226C374;
		  /* 8226C374h */ case   19:  		/* bl -10644 */
		/* 8226C374h case   19:*/		regs.LR = 0x8226C378; return 0x822699E0;
		/* 8226C374h case   19:*/		return 0x8226C378;
	}
	return 0x8226C378;
} // Block from 8226C328h-8226C378h (20 instructions)

//////////////////////////////////////////////////////
// Block at 8226C378h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C378( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C378) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C378);
		  /* 8226C378h */ case    0:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C378h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C378h case    0:*/		return 0x8226C37C;
		  /* 8226C37Ch */ case    1:  		/* rlwinm. R5, R11, 21, 31, 31 */
		/* 8226C37Ch case    1:*/		cpu::op::rlwinm<1,21,31,31>(regs,&regs.R5,regs.R11);
		/* 8226C37Ch case    1:*/		return 0x8226C380;
		  /* 8226C380h */ case    2:  		/* bc 12, CR0_EQ, 1320 */
		/* 8226C380h case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C380h case    2:*/		return 0x8226C384;
		  /* 8226C384h */ case    3:  		/* lis R11, -32252 */
		/* 8226C384h case    3:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8204);
		/* 8226C384h case    3:*/		return 0x8226C388;
		  /* 8226C388h */ case    4:  		/* addi R4, R11, -22320 */
		/* 8226C388h case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFA8D0);
		/* 8226C388h case    4:*/		return 0x8226C38C;
		  /* 8226C38Ch */ case    5:  		/* b 444 */
		/* 8226C38Ch case    5:*/		return 0x8226C548;
		/* 8226C38Ch case    5:*/		return 0x8226C390;
	}
	return 0x8226C390;
} // Block from 8226C378h-8226C390h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226C390h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C390( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C390) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C390);
		  /* 8226C390h */ case    0:  		/* lis R11, -32255 */
		/* 8226C390h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C390h case    0:*/		return 0x8226C394;
		  /* 8226C394h */ case    1:  		/* addi R5, R11, 5224 */
		/* 8226C394h case    1:*/		cpu::op::addi<0>(regs,&regs.R5,regs.R11,0x1468);
		/* 8226C394h case    1:*/		return 0x8226C398;
		  /* 8226C398h */ case    2:  		/* b -48 */
		/* 8226C398h case    2:*/		return 0x8226C368;
		/* 8226C398h case    2:*/		return 0x8226C39C;
		  /* 8226C39Ch */ case    3:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C39Ch case    3:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C39Ch case    3:*/		return 0x8226C3A0;
		  /* 8226C3A0h */ case    4:  		/* cmplwi CR0, R11, 0 */
		/* 8226C3A0h case    4:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C3A0h case    4:*/		return 0x8226C3A4;
		  /* 8226C3A4h */ case    5:  		/* bc 12, CR0_EQ, 1284 */
		/* 8226C3A4h case    5:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C3A4h case    5:*/		return 0x8226C3A8;
		  /* 8226C3A8h */ case    6:  		/* lis R11, -32254 */
		/* 8226C3A8h case    6:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C3A8h case    6:*/		return 0x8226C3AC;
		  /* 8226C3ACh */ case    7:  		/* mr R3, R31 */
		/* 8226C3ACh case    7:*/		regs.R3 = regs.R31;
		/* 8226C3ACh case    7:*/		return 0x8226C3B0;
		  /* 8226C3B0h */ case    8:  		/* addi R4, R11, -30880 */
		/* 8226C3B0h case    8:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C3B0h case    8:*/		return 0x8226C3B4;
		  /* 8226C3B4h */ case    9:  		/* bl -10708 */
		/* 8226C3B4h case    9:*/		regs.LR = 0x8226C3B8; return 0x822699E0;
		/* 8226C3B4h case    9:*/		return 0x8226C3B8;
		  /* 8226C3B8h */ case   10:  		/* lis R11, -32255 */
		/* 8226C3B8h case   10:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C3B8h case   10:*/		return 0x8226C3BC;
		  /* 8226C3BCh */ case   11:  		/* mr R3, R31 */
		/* 8226C3BCh case   11:*/		regs.R3 = regs.R31;
		/* 8226C3BCh case   11:*/		return 0x8226C3C0;
		  /* 8226C3C0h */ case   12:  		/* addi R4, R11, 12884 */
		/* 8226C3C0h case   12:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3254);
		/* 8226C3C0h case   12:*/		return 0x8226C3C4;
		  /* 8226C3C4h */ case   13:  		/* bl -10260 */
		/* 8226C3C4h case   13:*/		regs.LR = 0x8226C3C8; return 0x82269BB0;
		/* 8226C3C4h case   13:*/		return 0x8226C3C8;
		  /* 8226C3C8h */ case   14:  		/* lis R11, -32255 */
		/* 8226C3C8h case   14:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C3C8h case   14:*/		return 0x8226C3CC;
		  /* 8226C3CCh */ case   15:  		/* mr R3, R31 */
		/* 8226C3CCh case   15:*/		regs.R3 = regs.R31;
		/* 8226C3CCh case   15:*/		return 0x8226C3D0;
		  /* 8226C3D0h */ case   16:  		/* addi R4, R11, -15864 */
		/* 8226C3D0h case   16:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226C3D0h case   16:*/		return 0x8226C3D4;
		  /* 8226C3D4h */ case   17:  		/* bl -10740 */
		/* 8226C3D4h case   17:*/		regs.LR = 0x8226C3D8; return 0x822699E0;
		/* 8226C3D4h case   17:*/		return 0x8226C3D8;
		  /* 8226C3D8h */ case   18:  		/* lis R11, -32216 */
		/* 8226C3D8h case   18:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8228);
		/* 8226C3D8h case   18:*/		return 0x8226C3DC;
		  /* 8226C3DCh */ case   19:  		/* lis R10, -32255 */
		/* 8226C3DCh case   19:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8201);
		/* 8226C3DCh case   19:*/		return 0x8226C3E0;
		  /* 8226C3E0h */ case   20:  		/* addi R9, R11, 28528 */
		/* 8226C3E0h case   20:*/		cpu::op::addi<0>(regs,&regs.R9,regs.R11,0x6F70);
		/* 8226C3E0h case   20:*/		return 0x8226C3E4;
		  /* 8226C3E4h */ case   21:  		/* addi R4, R10, -10432 */
		/* 8226C3E4h case   21:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R10,0xFFFFD740);
		/* 8226C3E4h case   21:*/		return 0x8226C3E8;
		  /* 8226C3E8h */ case   22:  		/* mr R3, R31 */
		/* 8226C3E8h case   22:*/		regs.R3 = regs.R31;
		/* 8226C3E8h case   22:*/		return 0x8226C3EC;
		  /* 8226C3ECh */ case   23:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C3ECh case   23:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C3ECh case   23:*/		return 0x8226C3F0;
		  /* 8226C3F0h */ case   24:  		/* lwz R10, <#[R31 + 8]> */
		/* 8226C3F0h case   24:*/		cpu::mem::load32z( regs, &regs.R10, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C3F0h case   24:*/		return 0x8226C3F4;
		  /* 8226C3F4h */ case   25:  		/* rlwinm R30, R11, 23, 30, 31 */
		/* 8226C3F4h case   25:*/		cpu::op::rlwinm<0,23,30,31>(regs,&regs.R30,regs.R11);
		/* 8226C3F4h case   25:*/		return 0x8226C3F8;
		  /* 8226C3F8h */ case   26:  		/* rlwinm R10, R10, 29, 31, 31 */
		/* 8226C3F8h case   26:*/		cpu::op::rlwinm<0,29,31,31>(regs,&regs.R10,regs.R10);
		/* 8226C3F8h case   26:*/		return 0x8226C3FC;
		  /* 8226C3FCh */ case   27:  		/* rlwinm R11, R30, 1, 0, 30 */
		/* 8226C3FCh case   27:*/		cpu::op::rlwinm<0,1,0,30>(regs,&regs.R11,regs.R30);
		/* 8226C3FCh case   27:*/		return 0x8226C400;
		  /* 8226C400h */ case   28:  		/* add R11, R10, R11 */
		/* 8226C400h case   28:*/		cpu::op::add<0>(regs,&regs.R11,regs.R10,regs.R11);
		/* 8226C400h case   28:*/		return 0x8226C404;
		  /* 8226C404h */ case   29:  		/* rlwinm R11, R11, 2, 0, 29 */
		/* 8226C404h case   29:*/		cpu::op::rlwinm<0,2,0,29>(regs,&regs.R11,regs.R11);
		/* 8226C404h case   29:*/		return 0x8226C408;
		  /* 8226C408h */ case   30:  		/* lwzx R5, <#[R11 + R9]> */
		/* 8226C408h case   30:*/		cpu::mem::load32z( regs, &regs.R5, (uint32)(regs.R11 + regs.R9 + 0x00000000) );
		/* 8226C408h case   30:*/		return 0x8226C40C;
		  /* 8226C40Ch */ case   31:  		/* bl -10796 */
		/* 8226C40Ch case   31:*/		regs.LR = 0x8226C410; return 0x822699E0;
		/* 8226C40Ch case   31:*/		return 0x8226C410;
		  /* 8226C410h */ case   32:  		/* cmplwi CR6, R30, 3 */
		/* 8226C410h case   32:*/		cpu::op::cmplwi<6>(regs,regs.R30,0x00000003);
		/* 8226C410h case   32:*/		return 0x8226C414;
		  /* 8226C414h */ case   33:  		/* bc 4, CR6_EQ, 28 */
		/* 8226C414h case   33:*/		if ( !regs.CR[6].eq ) { return 0x8226C430;  }
		/* 8226C414h case   33:*/		return 0x8226C418;
		  /* 8226C418h */ case   34:  		/* lwz R11, <#[R28]> */
		/* 8226C418h case   34:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C418h case   34:*/		return 0x8226C41C;
		  /* 8226C41Ch */ case   35:  		/* lis R10, -32251 */
		/* 8226C41Ch case   35:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8205);
		/* 8226C41Ch case   35:*/		return 0x8226C420;
		  /* 8226C420h */ case   36:  		/* mr R3, R31 */
		/* 8226C420h case   36:*/		regs.R3 = regs.R31;
		/* 8226C420h case   36:*/		return 0x8226C424;
		  /* 8226C424h */ case   37:  		/* addi R4, R10, -15900 */
		/* 8226C424h case   37:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R10,0xFFFFC1E4);
		/* 8226C424h case   37:*/		return 0x8226C428;
		  /* 8226C428h */ case   38:  		/* rlwinm R5, R11, 0, 29, 31 */
		/* 8226C428h case   38:*/		cpu::op::rlwinm<0,0,29,31>(regs,&regs.R5,regs.R11);
		/* 8226C428h case   38:*/		return 0x8226C42C;
		  /* 8226C42Ch */ case   39:  		/* bl -10828 */
		/* 8226C42Ch case   39:*/		regs.LR = 0x8226C430; return 0x822699E0;
		/* 8226C42Ch case   39:*/		return 0x8226C430;
	}
	return 0x8226C430;
} // Block from 8226C390h-8226C430h (40 instructions)

//////////////////////////////////////////////////////
// Block at 8226C430h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C430( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C430) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C430);
		  /* 8226C430h */ case    0:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C430h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C430h case    0:*/		return 0x8226C434;
		  /* 8226C434h */ case    1:  		/* rlwinm. R11, R11, 0, 23, 23 */
		/* 8226C434h case    1:*/		cpu::op::rlwinm<1,0,23,23>(regs,&regs.R11,regs.R11);
		/* 8226C434h case    1:*/		return 0x8226C438;
		  /* 8226C438h */ case    2:  		/* bc 12, CR0_EQ, 1136 */
		/* 8226C438h case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C438h case    2:*/		return 0x8226C43C;
		  /* 8226C43Ch */ case    3:  		/* lis R11, -32251 */
		/* 8226C43Ch case    3:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C43Ch case    3:*/		return 0x8226C440;
		  /* 8226C440h */ case    4:  		/* addi R4, R11, -15924 */
		/* 8226C440h case    4:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC1CC);
		/* 8226C440h case    4:*/		return 0x8226C444;
		  /* 8226C444h */ case    5:  		/* b -856 */
		/* 8226C444h case    5:*/		return 0x8226C0EC;
		/* 8226C444h case    5:*/		return 0x8226C448;
		  /* 8226C448h */ case    6:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C448h case    6:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C448h case    6:*/		return 0x8226C44C;
		  /* 8226C44Ch */ case    7:  		/* cmplwi CR0, R11, 0 */
		/* 8226C44Ch case    7:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C44Ch case    7:*/		return 0x8226C450;
		  /* 8226C450h */ case    8:  		/* bc 12, CR0_EQ, 1112 */
		/* 8226C450h case    8:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C450h case    8:*/		return 0x8226C454;
		  /* 8226C454h */ case    9:  		/* lis R11, -32254 */
		/* 8226C454h case    9:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C454h case    9:*/		return 0x8226C458;
		  /* 8226C458h */ case   10:  		/* mr R3, R31 */
		/* 8226C458h case   10:*/		regs.R3 = regs.R31;
		/* 8226C458h case   10:*/		return 0x8226C45C;
		  /* 8226C45Ch */ case   11:  		/* addi R4, R11, -30880 */
		/* 8226C45Ch case   11:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C45Ch case   11:*/		return 0x8226C460;
		  /* 8226C460h */ case   12:  		/* bl -10880 */
		/* 8226C460h case   12:*/		regs.LR = 0x8226C464; return 0x822699E0;
		/* 8226C460h case   12:*/		return 0x8226C464;
		  /* 8226C464h */ case   13:  		/* lis R11, -32255 */
		/* 8226C464h case   13:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C464h case   13:*/		return 0x8226C468;
		  /* 8226C468h */ case   14:  		/* addi R4, R11, 12920 */
		/* 8226C468h case   14:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3278);
		/* 8226C468h case   14:*/		return 0x8226C46C;
		  /* 8226C46Ch */ case   15:  		/* mr R3, R31 */
		/* 8226C46Ch case   15:*/		regs.R3 = regs.R31;
		/* 8226C46Ch case   15:*/		return 0x8226C470;
		  /* 8226C470h */ case   16:  		/* bl -10432 */
		/* 8226C470h case   16:*/		regs.LR = 0x8226C474; return 0x82269BB0;
		/* 8226C470h case   16:*/		return 0x8226C474;
		  /* 8226C474h */ case   17:  		/* lis R11, -32255 */
		/* 8226C474h case   17:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C474h case   17:*/		return 0x8226C478;
		  /* 8226C478h */ case   18:  		/* mr R3, R31 */
		/* 8226C478h case   18:*/		regs.R3 = regs.R31;
		/* 8226C478h case   18:*/		return 0x8226C47C;
		  /* 8226C47Ch */ case   19:  		/* addi R4, R11, -15864 */
		/* 8226C47Ch case   19:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC208);
		/* 8226C47Ch case   19:*/		return 0x8226C480;
		  /* 8226C480h */ case   20:  		/* bl -10912 */
		/* 8226C480h case   20:*/		regs.LR = 0x8226C484; return 0x822699E0;
		/* 8226C480h case   20:*/		return 0x8226C484;
		  /* 8226C484h */ case   21:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C484h case   21:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C484h case   21:*/		return 0x8226C488;
		  /* 8226C488h */ case   22:  		/* rlwinm. R11, R11, 0, 21, 21 */
		/* 8226C488h case   22:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R11);
		/* 8226C488h case   22:*/		return 0x8226C48C;
		  /* 8226C48Ch */ case   23:  		/* bc 4, CR0_EQ, 20 */
		/* 8226C48Ch case   23:*/		if ( !regs.CR[0].eq ) { return 0x8226C4A0;  }
		/* 8226C48Ch case   23:*/		return 0x8226C490;
		  /* 8226C490h */ case   24:  		/* lis R11, -32255 */
		/* 8226C490h case   24:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C490h case   24:*/		return 0x8226C494;
		  /* 8226C494h */ case   25:  		/* mr R3, R31 */
		/* 8226C494h case   25:*/		regs.R3 = regs.R31;
		/* 8226C494h case   25:*/		return 0x8226C498;
		  /* 8226C498h */ case   26:  		/* addi R4, R11, -16488 */
		/* 8226C498h case   26:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFBF98);
		/* 8226C498h case   26:*/		return 0x8226C49C;
		  /* 8226C49Ch */ case   27:  		/* bl -10940 */
		/* 8226C49Ch case   27:*/		regs.LR = 0x8226C4A0; return 0x822699E0;
		/* 8226C49Ch case   27:*/		return 0x8226C4A0;
	}
	return 0x8226C4A0;
} // Block from 8226C430h-8226C4A0h (28 instructions)

//////////////////////////////////////////////////////
// Block at 8226C4A0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C4A0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C4A0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C4A0);
		  /* 8226C4A0h */ case    0:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C4A0h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C4A0h case    0:*/		return 0x8226C4A4;
		  /* 8226C4A4h */ case    1:  		/* lis R10, -32252 */
		/* 8226C4A4h case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8204);
		/* 8226C4A4h case    1:*/		return 0x8226C4A8;
		  /* 8226C4A8h */ case    2:  		/* mr R3, R31 */
		/* 8226C4A8h case    2:*/		regs.R3 = regs.R31;
		/* 8226C4A8h case    2:*/		return 0x8226C4AC;
		  /* 8226C4ACh */ case    3:  		/* addi R4, R10, -22336 */
		/* 8226C4ACh case    3:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R10,0xFFFFA8C0);
		/* 8226C4ACh case    3:*/		return 0x8226C4B0;
		  /* 8226C4B0h */ case    4:  		/* rlwinm R5, R11, 30, 24, 31 */
		/* 8226C4B0h case    4:*/		cpu::op::rlwinm<0,30,24,31>(regs,&regs.R5,regs.R11);
		/* 8226C4B0h case    4:*/		return 0x8226C4B4;
		  /* 8226C4B4h */ case    5:  		/* bl -10964 */
		/* 8226C4B4h case    5:*/		regs.LR = 0x8226C4B8; return 0x822699E0;
		/* 8226C4B4h case    5:*/		return 0x8226C4B8;
		  /* 8226C4B8h */ case    6:  		/* li R5, 1 */
		/* 8226C4B8h case    6:*/		cpu::op::li<0>(regs,&regs.R5,0x1);
		/* 8226C4B8h case    6:*/		return 0x8226C4BC;
		  /* 8226C4BCh */ case    7:  		/* b -1508 */
		/* 8226C4BCh case    7:*/		return 0x8226BED8;
		/* 8226C4BCh case    7:*/		return 0x8226C4C0;
		  /* 8226C4C0h */ case    8:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C4C0h case    8:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C4C0h case    8:*/		return 0x8226C4C4;
		  /* 8226C4C4h */ case    9:  		/* cmplwi CR0, R11, 0 */
		/* 8226C4C4h case    9:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C4C4h case    9:*/		return 0x8226C4C8;
		  /* 8226C4C8h */ case   10:  		/* bc 12, CR0_EQ, 992 */
		/* 8226C4C8h case   10:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C4C8h case   10:*/		return 0x8226C4CC;
		  /* 8226C4CCh */ case   11:  		/* lis R11, -32254 */
		/* 8226C4CCh case   11:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C4CCh case   11:*/		return 0x8226C4D0;
		  /* 8226C4D0h */ case   12:  		/* mr R3, R31 */
		/* 8226C4D0h case   12:*/		regs.R3 = regs.R31;
		/* 8226C4D0h case   12:*/		return 0x8226C4D4;
		  /* 8226C4D4h */ case   13:  		/* addi R4, R11, -30880 */
		/* 8226C4D4h case   13:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C4D4h case   13:*/		return 0x8226C4D8;
		  /* 8226C4D8h */ case   14:  		/* bl -11000 */
		/* 8226C4D8h case   14:*/		regs.LR = 0x8226C4DC; return 0x822699E0;
		/* 8226C4D8h case   14:*/		return 0x8226C4DC;
		  /* 8226C4DCh */ case   15:  		/* lis R11, -32255 */
		/* 8226C4DCh case   15:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C4DCh case   15:*/		return 0x8226C4E0;
		  /* 8226C4E0h */ case   16:  		/* addi R4, R11, 12912 */
		/* 8226C4E0h case   16:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x3270);
		/* 8226C4E0h case   16:*/		return 0x8226C4E4;
		  /* 8226C4E4h */ case   17:  		/* b -120 */
		/* 8226C4E4h case   17:*/		return 0x8226C46C;
		/* 8226C4E4h case   17:*/		return 0x8226C4E8;
		  /* 8226C4E8h */ case   18:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C4E8h case   18:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C4E8h case   18:*/		return 0x8226C4EC;
		  /* 8226C4ECh */ case   19:  		/* cmplwi CR0, R11, 0 */
		/* 8226C4ECh case   19:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C4ECh case   19:*/		return 0x8226C4F0;
		  /* 8226C4F0h */ case   20:  		/* bc 12, CR0_EQ, 952 */
		/* 8226C4F0h case   20:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C4F0h case   20:*/		return 0x8226C4F4;
		  /* 8226C4F4h */ case   21:  		/* lis R11, -32254 */
		/* 8226C4F4h case   21:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C4F4h case   21:*/		return 0x8226C4F8;
		  /* 8226C4F8h */ case   22:  		/* mr R3, R31 */
		/* 8226C4F8h case   22:*/		regs.R3 = regs.R31;
		/* 8226C4F8h case   22:*/		return 0x8226C4FC;
		  /* 8226C4FCh */ case   23:  		/* addi R4, R11, -30880 */
		/* 8226C4FCh case   23:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C4FCh case   23:*/		return 0x8226C500;
		  /* 8226C500h */ case   24:  		/* bl -11040 */
		/* 8226C500h case   24:*/		regs.LR = 0x8226C504; return 0x822699E0;
		/* 8226C500h case   24:*/		return 0x8226C504;
		  /* 8226C504h */ case   25:  		/* lis R11, -32255 */
		/* 8226C504h case   25:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C504h case   25:*/		return 0x8226C508;
		  /* 8226C508h */ case   26:  		/* addi R4, R11, 12876 */
		/* 8226C508h case   26:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x324C);
		/* 8226C508h case   26:*/		return 0x8226C50C;
		  /* 8226C50Ch */ case   27:  		/* b -1832 */
		/* 8226C50Ch case   27:*/		return 0x8226BDE4;
		/* 8226C50Ch case   27:*/		return 0x8226C510;
	}
	return 0x8226C510;
} // Block from 8226C4A0h-8226C510h (28 instructions)

//////////////////////////////////////////////////////
// Block at 8226C510h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C510( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C510) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C510);
		  /* 8226C510h */ case    0:  		/* lbz R11, <#[R31 + 12]> */
		/* 8226C510h case    0:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x0000000C) );
		/* 8226C510h case    0:*/		return 0x8226C514;
		  /* 8226C514h */ case    1:  		/* cmplwi CR0, R11, 0 */
		/* 8226C514h case    1:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C514h case    1:*/		return 0x8226C518;
		  /* 8226C518h */ case    2:  		/* bc 12, CR0_EQ, 912 */
		/* 8226C518h case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C518h case    2:*/		return 0x8226C51C;
		  /* 8226C51Ch */ case    3:  		/* lis R11, -32254 */
		/* 8226C51Ch case    3:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C51Ch case    3:*/		return 0x8226C520;
		  /* 8226C520h */ case    4:  		/* mr R3, R31 */
		/* 8226C520h case    4:*/		regs.R3 = regs.R31;
		/* 8226C520h case    4:*/		return 0x8226C524;
		  /* 8226C524h */ case    5:  		/* addi R4, R11, -30880 */
		/* 8226C524h case    5:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF8760);
		/* 8226C524h case    5:*/		return 0x8226C528;
		  /* 8226C528h */ case    6:  		/* bl -11080 */
		/* 8226C528h case    6:*/		regs.LR = 0x8226C52C; return 0x822699E0;
		/* 8226C528h case    6:*/		return 0x8226C52C;
		  /* 8226C52Ch */ case    7:  		/* lis R11, -32252 */
		/* 8226C52Ch case    7:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8204);
		/* 8226C52Ch case    7:*/		return 0x8226C530;
		  /* 8226C530h */ case    8:  		/* mr R3, R31 */
		/* 8226C530h case    8:*/		regs.R3 = regs.R31;
		/* 8226C530h case    8:*/		return 0x8226C534;
		  /* 8226C534h */ case    9:  		/* addi R4, R11, -22252 */
		/* 8226C534h case    9:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFA914);
		/* 8226C534h case    9:*/		return 0x8226C538;
		  /* 8226C538h */ case   10:  		/* bl -10632 */
		/* 8226C538h case   10:*/		regs.LR = 0x8226C53C; return 0x82269BB0;
		/* 8226C538h case   10:*/		return 0x8226C53C;
		  /* 8226C53Ch */ case   11:  		/* lis R11, -32252 */
		/* 8226C53Ch case   11:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8204);
		/* 8226C53Ch case   11:*/		return 0x8226C540;
		  /* 8226C540h */ case   12:  		/* mr R5, R30 */
		/* 8226C540h case   12:*/		regs.R5 = regs.R30;
		/* 8226C540h case   12:*/		return 0x8226C544;
		  /* 8226C544h */ case   13:  		/* addi R4, R11, -22332 */
		/* 8226C544h case   13:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFA8C4);
		/* 8226C544h case   13:*/		return 0x8226C548;
	}
	return 0x8226C548;
} // Block from 8226C510h-8226C548h (14 instructions)

//////////////////////////////////////////////////////
// Block at 8226C548h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C548( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C548) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C548);
		  /* 8226C548h */ case    0:  		/* mr R3, R31 */
		/* 8226C548h case    0:*/		regs.R3 = regs.R31;
		/* 8226C548h case    0:*/		return 0x8226C54C;
		  /* 8226C54Ch */ case    1:  		/* bl -11116 */
		/* 8226C54Ch case    1:*/		regs.LR = 0x8226C550; return 0x822699E0;
		/* 8226C54Ch case    1:*/		return 0x8226C550;
		  /* 8226C550h */ case    2:  		/* b 856 */
		/* 8226C550h case    2:*/		return 0x8226C8A8;
		/* 8226C550h case    2:*/		return 0x8226C554;
	}
	return 0x8226C554;
} // Block from 8226C548h-8226C554h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C554h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C554( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C554) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C554);
		  /* 8226C554h */ case    0:  		/* rlwinm. R11, R29, 0, 29, 30 */
		/* 8226C554h case    0:*/		cpu::op::rlwinm<1,0,29,30>(regs,&regs.R11,regs.R29);
		/* 8226C554h case    0:*/		return 0x8226C558;
		  /* 8226C558h */ case    1:  		/* bc 12, CR0_EQ, 848 */
		/* 8226C558h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C558h case    1:*/		return 0x8226C55C;
		  /* 8226C55Ch */ case    2:  		/* lwz R11, <#[R31 + 8240]> */
		/* 8226C55Ch case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00002030) );
		/* 8226C55Ch case    2:*/		return 0x8226C560;
		  /* 8226C560h */ case    3:  		/* cmplw CR6, R11, R22 */
		/* 8226C560h case    3:*/		cpu::op::cmplw<6>(regs,regs.R11,regs.R22);
		/* 8226C560h case    3:*/		return 0x8226C564;
		  /* 8226C564h */ case    4:  		/* bc 12, CR6_GT, 8 */
		/* 8226C564h case    4:*/		if ( regs.CR[6].gt ) { return 0x8226C56C;  }
		/* 8226C564h case    4:*/		return 0x8226C568;
		  /* 8226C568h */ case    5:  		/* mr R11, R22 */
		/* 8226C568h case    5:*/		regs.R11 = regs.R22;
		/* 8226C568h case    5:*/		return 0x8226C56C;
	}
	return 0x8226C56C;
} // Block from 8226C554h-8226C56Ch (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226C56Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C56C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C56C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C56C);
		  /* 8226C56Ch */ case    0:  		/* rlwinm. R10, R29, 0, 30, 30 */
		/* 8226C56Ch case    0:*/		cpu::op::rlwinm<1,0,30,30>(regs,&regs.R10,regs.R29);
		/* 8226C56Ch case    0:*/		return 0x8226C570;
		  /* 8226C570h */ case    1:  		/* stw R11, <#[R31 + 8240]> */
		/* 8226C570h case    1:*/		cpu::mem::store32( regs, regs.R11, (uint32)(regs.R31 + 0x00002030) );
		/* 8226C570h case    1:*/		return 0x8226C574;
		  /* 8226C574h */ case    2:  		/* rlwinm R21, R29, 27, 31, 31 */
		/* 8226C574h case    2:*/		cpu::op::rlwinm<0,27,31,31>(regs,&regs.R21,regs.R29);
		/* 8226C574h case    2:*/		return 0x8226C578;
		  /* 8226C578h */ case    3:  		/* bc 12, CR0_EQ, 172 */
		/* 8226C578h case    3:*/		if ( regs.CR[0].eq ) { return 0x8226C624;  }
		/* 8226C578h case    3:*/		return 0x8226C57C;
		  /* 8226C57Ch */ case    4:  		/* mr R3, R31 */
		/* 8226C57Ch case    4:*/		regs.R3 = regs.R31;
		/* 8226C57Ch case    4:*/		return 0x8226C580;
		  /* 8226C580h */ case    5:  		/* bl -6336 */
		/* 8226C580h case    5:*/		regs.LR = 0x8226C584; return 0x8226ACC0;
		/* 8226C580h case    5:*/		return 0x8226C584;
		  /* 8226C584h */ case    6:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226C584h case    6:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C584h case    6:*/		return 0x8226C588;
		  /* 8226C588h */ case    7:  		/* rlwinm. R11, R11, 0, 26, 26 */
		/* 8226C588h case    7:*/		cpu::op::rlwinm<1,0,26,26>(regs,&regs.R11,regs.R11);
		/* 8226C588h case    7:*/		return 0x8226C58C;
		  /* 8226C58Ch */ case    8:  		/* bc 12, CR0_EQ, 24 */
		/* 8226C58Ch case    8:*/		if ( regs.CR[0].eq ) { return 0x8226C5A4;  }
		/* 8226C58Ch case    8:*/		return 0x8226C590;
		  /* 8226C590h */ case    9:  		/* lis R11, -32251 */
		/* 8226C590h case    9:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C590h case    9:*/		return 0x8226C594;
		  /* 8226C594h */ case   10:  		/* mr R5, R22 */
		/* 8226C594h case   10:*/		regs.R5 = regs.R22;
		/* 8226C594h case   10:*/		return 0x8226C598;
		  /* 8226C598h */ case   11:  		/* addi R4, R11, -16304 */
		/* 8226C598h case   11:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC050);
		/* 8226C598h case   11:*/		return 0x8226C59C;
		  /* 8226C59Ch */ case   12:  		/* mr R3, R31 */
		/* 8226C59Ch case   12:*/		regs.R3 = regs.R31;
		/* 8226C59Ch case   12:*/		return 0x8226C5A0;
		  /* 8226C5A0h */ case   13:  		/* bl -11200 */
		/* 8226C5A0h case   13:*/		regs.LR = 0x8226C5A4; return 0x822699E0;
		/* 8226C5A0h case   13:*/		return 0x8226C5A4;
	}
	return 0x8226C5A4;
} // Block from 8226C56Ch-8226C5A4h (14 instructions)

//////////////////////////////////////////////////////
// Block at 8226C5A4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C5A4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C5A4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C5A4);
		  /* 8226C5A4h */ case    0:  		/* lis R11, -32254 */
		/* 8226C5A4h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8202);
		/* 8226C5A4h case    0:*/		return 0x8226C5A8;
		  /* 8226C5A8h */ case    1:  		/* rlwinm. R10, R21, 0, 24, 31 */
		/* 8226C5A8h case    1:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R10,regs.R21);
		/* 8226C5A8h case    1:*/		return 0x8226C5AC;
		  /* 8226C5ACh */ case    2:  		/* addi R30, R11, -30880 */
		/* 8226C5ACh case    2:*/		cpu::op::addi<0>(regs,&regs.R30,regs.R11,0xFFFF8760);
		/* 8226C5ACh case    2:*/		return 0x8226C5B0;
		  /* 8226C5B0h */ case    3:  		/* bc 12, CR0_EQ, 80 */
		/* 8226C5B0h case    3:*/		if ( regs.CR[0].eq ) { return 0x8226C600;  }
		/* 8226C5B0h case    3:*/		return 0x8226C5B4;
		  /* 8226C5B4h */ case    4:  		/* mr R4, R30 */
		/* 8226C5B4h case    4:*/		regs.R4 = regs.R30;
		/* 8226C5B4h case    4:*/		return 0x8226C5B8;
		  /* 8226C5B8h */ case    5:  		/* mr R3, R31 */
		/* 8226C5B8h case    5:*/		regs.R3 = regs.R31;
		/* 8226C5B8h case    5:*/		return 0x8226C5BC;
		  /* 8226C5BCh */ case    6:  		/* bl -11228 */
		/* 8226C5BCh case    6:*/		regs.LR = 0x8226C5C0; return 0x822699E0;
		/* 8226C5BCh case    6:*/		return 0x8226C5C0;
		  /* 8226C5C0h */ case    7:  		/* lis R11, -32255 */
		/* 8226C5C0h case    7:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226C5C0h case    7:*/		return 0x8226C5C4;
		  /* 8226C5C4h */ case    8:  		/* mr R3, R31 */
		/* 8226C5C4h case    8:*/		regs.R3 = regs.R31;
		/* 8226C5C4h case    8:*/		return 0x8226C5C8;
		  /* 8226C5C8h */ case    9:  		/* addi R4, R11, 28524 */
		/* 8226C5C8h case    9:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0x6F6C);
		/* 8226C5C8h case    9:*/		return 0x8226C5CC;
		  /* 8226C5CCh */ case   10:  		/* bl -10780 */
		/* 8226C5CCh case   10:*/		regs.LR = 0x8226C5D0; return 0x82269BB0;
		/* 8226C5CCh case   10:*/		return 0x8226C5D0;
		  /* 8226C5D0h */ case   11:  		/* lbz R11, <#[R31 + 8232]> */
		/* 8226C5D0h case   11:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x00002028) );
		/* 8226C5D0h case   11:*/		return 0x8226C5D4;
		  /* 8226C5D4h */ case   12:  		/* cmplwi CR0, R11, 0 */
		/* 8226C5D4h case   12:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C5D4h case   12:*/		return 0x8226C5D8;
		  /* 8226C5D8h */ case   13:  		/* bc 4, CR0_EQ, 12 */
		/* 8226C5D8h case   13:*/		if ( !regs.CR[0].eq ) { return 0x8226C5E4;  }
		/* 8226C5D8h case   13:*/		return 0x8226C5DC;
		  /* 8226C5DCh */ case   14:  		/* mr R3, R31 */
		/* 8226C5DCh case   14:*/		regs.R3 = regs.R31;
		/* 8226C5DCh case   14:*/		return 0x8226C5E0;
		  /* 8226C5E0h */ case   15:  		/* bl -6520 */
		/* 8226C5E0h case   15:*/		regs.LR = 0x8226C5E4; return 0x8226AC68;
		/* 8226C5E0h case   15:*/		return 0x8226C5E4;
	}
	return 0x8226C5E4;
} // Block from 8226C5A4h-8226C5E4h (16 instructions)

//////////////////////////////////////////////////////
// Block at 8226C5E4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C5E4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C5E4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C5E4);
		  /* 8226C5E4h */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226C5E4h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C5E4h case    0:*/		return 0x8226C5E8;
		  /* 8226C5E8h */ case    1:  		/* rlwinm. R11, R11, 0, 26, 26 */
		/* 8226C5E8h case    1:*/		cpu::op::rlwinm<1,0,26,26>(regs,&regs.R11,regs.R11);
		/* 8226C5E8h case    1:*/		return 0x8226C5EC;
		  /* 8226C5ECh */ case    2:  		/* bc 12, CR0_EQ, 20 */
		/* 8226C5ECh case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C600;  }
		/* 8226C5ECh case    2:*/		return 0x8226C5F0;
		  /* 8226C5F0h */ case    3:  		/* lis R11, -32251 */
		/* 8226C5F0h case    3:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C5F0h case    3:*/		return 0x8226C5F4;
		  /* 8226C5F4h */ case    4:  		/* mr R3, R31 */
		/* 8226C5F4h case    4:*/		regs.R3 = regs.R31;
		/* 8226C5F4h case    4:*/		return 0x8226C5F8;
		  /* 8226C5F8h */ case    5:  		/* addi R4, R11, -16288 */
		/* 8226C5F8h case    5:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC060);
		/* 8226C5F8h case    5:*/		return 0x8226C5FC;
		  /* 8226C5FCh */ case    6:  		/* bl -11292 */
		/* 8226C5FCh case    6:*/		regs.LR = 0x8226C600; return 0x822699E0;
		/* 8226C5FCh case    6:*/		return 0x8226C600;
	}
	return 0x8226C600;
} // Block from 8226C5E4h-8226C600h (7 instructions)

//////////////////////////////////////////////////////
// Block at 8226C600h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C600( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C600) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C600);
		  /* 8226C600h */ case    0:  		/* mr R4, R30 */
		/* 8226C600h case    0:*/		regs.R4 = regs.R30;
		/* 8226C600h case    0:*/		return 0x8226C604;
		  /* 8226C604h */ case    1:  		/* mr R3, R31 */
		/* 8226C604h case    1:*/		regs.R3 = regs.R31;
		/* 8226C604h case    1:*/		return 0x8226C608;
		  /* 8226C608h */ case    2:  		/* bl -11304 */
		/* 8226C608h case    2:*/		regs.LR = 0x8226C60C; return 0x822699E0;
		/* 8226C608h case    2:*/		return 0x8226C60C;
		  /* 8226C60Ch */ case    3:  		/* mr R6, R22 */
		/* 8226C60Ch case    3:*/		regs.R6 = regs.R22;
		/* 8226C60Ch case    3:*/		return 0x8226C610;
		  /* 8226C610h */ case    4:  		/* rlwinm R5, R29, 28, 31, 31 */
		/* 8226C610h case    4:*/		cpu::op::rlwinm<0,28,31,31>(regs,&regs.R5,regs.R29);
		/* 8226C610h case    4:*/		return 0x8226C614;
		  /* 8226C614h */ case    5:  		/* mr R4, R28 */
		/* 8226C614h case    5:*/		regs.R4 = regs.R28;
		/* 8226C614h case    5:*/		return 0x8226C618;
		  /* 8226C618h */ case    6:  		/* mr R3, R31 */
		/* 8226C618h case    6:*/		regs.R3 = regs.R31;
		/* 8226C618h case    6:*/		return 0x8226C61C;
		  /* 8226C61Ch */ case    7:  		/* bl -5988 */
		/* 8226C61Ch case    7:*/		regs.LR = 0x8226C620; return 0x8226AEB8;
		/* 8226C61Ch case    7:*/		return 0x8226C620;
		  /* 8226C620h */ case    8:  		/* b 648 */
		/* 8226C620h case    8:*/		return 0x8226C8A8;
		/* 8226C620h case    8:*/		return 0x8226C624;
	}
	return 0x8226C624;
} // Block from 8226C600h-8226C624h (9 instructions)

//////////////////////////////////////////////////////
// Block at 8226C624h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C624( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C624) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C624);
		  /* 8226C624h */ case    0:  		/* rlwinm. R11, R29, 0, 29, 29 */
		/* 8226C624h case    0:*/		cpu::op::rlwinm<1,0,29,29>(regs,&regs.R11,regs.R29);
		/* 8226C624h case    0:*/		return 0x8226C628;
		  /* 8226C628h */ case    1:  		/* bc 12, CR0_EQ, 640 */
		/* 8226C628h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C628h case    1:*/		return 0x8226C62C;
		  /* 8226C62Ch */ case    2:  		/* mr R3, R28 */
		/* 8226C62Ch case    2:*/		regs.R3 = regs.R28;
		/* 8226C62Ch case    2:*/		return 0x8226C630;
		  /* 8226C630h */ case    3:  		/* bl -11712 */
		/* 8226C630h case    3:*/		regs.LR = 0x8226C634; return 0x82269870;
		/* 8226C630h case    3:*/		return 0x8226C634;
		  /* 8226C634h */ case    4:  		/* lis R11, -32251 */
		/* 8226C634h case    4:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C634h case    4:*/		return 0x8226C638;
		  /* 8226C638h */ case    5:  		/* lis R10, -32255 */
		/* 8226C638h case    5:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8201);
		/* 8226C638h case    5:*/		return 0x8226C63C;
		  /* 8226C63Ch */ case    6:  		/* lis R9, -32251 */
		/* 8226C63Ch case    6:*/		cpu::op::lis<0>(regs,&regs.R9,0xFFFF8205);
		/* 8226C63Ch case    6:*/		return 0x8226C640;
		  /* 8226C640h */ case    7:  		/* lis R8, -32251 */
		/* 8226C640h case    7:*/		cpu::op::lis<0>(regs,&regs.R8,0xFFFF8205);
		/* 8226C640h case    7:*/		return 0x8226C644;
		  /* 8226C644h */ case    8:  		/* lis R7, -32251 */
		/* 8226C644h case    8:*/		cpu::op::lis<0>(regs,&regs.R7,0xFFFF8205);
		/* 8226C644h case    8:*/		return 0x8226C648;
		  /* 8226C648h */ case    9:  		/* lis R6, -32254 */
		/* 8226C648h case    9:*/		cpu::op::lis<0>(regs,&regs.R6,0xFFFF8202);
		/* 8226C648h case    9:*/		return 0x8226C64C;
		  /* 8226C64Ch */ case   10:  		/* rlwinm. R5, R3, 0, 24, 31 */
		/* 8226C64Ch case   10:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R5,regs.R3);
		/* 8226C64Ch case   10:*/		return 0x8226C650;
		  /* 8226C650h */ case   11:  		/* lis R27, -14336 */
		/* 8226C650h case   11:*/		cpu::op::lis<0>(regs,&regs.R27,0xFFFFC800);
		/* 8226C650h case   11:*/		return 0x8226C654;
		  /* 8226C654h */ case   12:  		/* addi R25, R11, -16288 */
		/* 8226C654h case   12:*/		cpu::op::addi<0>(regs,&regs.R25,regs.R11,0xFFFFC060);
		/* 8226C654h case   12:*/		return 0x8226C658;
		  /* 8226C658h */ case   13:  		/* addi R26, R10, 28524 */
		/* 8226C658h case   13:*/		cpu::op::addi<0>(regs,&regs.R26,regs.R10,0x6F6C);
		/* 8226C658h case   13:*/		return 0x8226C65C;
		  /* 8226C65Ch */ case   14:  		/* addi R29, R9, -16304 */
		/* 8226C65Ch case   14:*/		cpu::op::addi<0>(regs,&regs.R29,regs.R9,0xFFFFC050);
		/* 8226C65Ch case   14:*/		return 0x8226C660;
		  /* 8226C660h */ case   15:  		/* addi R24, R8, -16352 */
		/* 8226C660h case   15:*/		cpu::op::addi<0>(regs,&regs.R24,regs.R8,0xFFFFC020);
		/* 8226C660h case   15:*/		return 0x8226C664;
		  /* 8226C664h */ case   16:  		/* addi R23, R7, -16344 */
		/* 8226C664h case   16:*/		cpu::op::addi<0>(regs,&regs.R23,regs.R7,0xFFFFC028);
		/* 8226C664h case   16:*/		return 0x8226C668;
		  /* 8226C668h */ case   17:  		/* addi R30, R6, -30880 */
		/* 8226C668h case   17:*/		cpu::op::addi<0>(regs,&regs.R30,regs.R6,0xFFFF8760);
		/* 8226C668h case   17:*/		return 0x8226C66C;
		  /* 8226C66Ch */ case   18:  		/* bc 12, CR0_EQ, 72 */
		/* 8226C66Ch case   18:*/		if ( regs.CR[0].eq ) { return 0x8226C6B4;  }
		/* 8226C66Ch case   18:*/		return 0x8226C670;
		  /* 8226C670h */ case   19:  		/* lwz R11, <#[R28]> */
		/* 8226C670h case   19:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C670h case   19:*/		return 0x8226C674;
		  /* 8226C674h */ case   20:  		/* rlwinm R10, R11, 0, 0, 5 */
		/* 8226C674h case   20:*/		cpu::op::rlwinm<0,0,0,5>(regs,&regs.R10,regs.R11);
		/* 8226C674h case   20:*/		return 0x8226C678;
		  /* 8226C678h */ case   21:  		/* cmplw CR6, R10, R27 */
		/* 8226C678h case   21:*/		cpu::op::cmplw<6>(regs,regs.R10,regs.R27);
		/* 8226C678h case   21:*/		return 0x8226C67C;
		  /* 8226C67Ch */ case   22:  		/* bc 4, CR6_EQ, 44 */
		/* 8226C67Ch case   22:*/		if ( !regs.CR[6].eq ) { return 0x8226C6A8;  }
		/* 8226C67Ch case   22:*/		return 0x8226C680;
		  /* 8226C680h */ case   23:  		/* rlwinm. R10, R11, 0, 16, 16 */
		/* 8226C680h case   23:*/		cpu::op::rlwinm<1,0,16,16>(regs,&regs.R10,regs.R11);
		/* 8226C680h case   23:*/		return 0x8226C684;
		  /* 8226C684h */ case   24:  		/* bc 12, CR0_EQ, 24 */
		/* 8226C684h case   24:*/		if ( regs.CR[0].eq ) { return 0x8226C69C;  }
		/* 8226C684h case   24:*/		return 0x8226C688;
		  /* 8226C688h */ case   25:  		/* mr R3, R28 */
		/* 8226C688h case   25:*/		regs.R3 = regs.R28;
		/* 8226C688h case   25:*/		return 0x8226C68C;
		  /* 8226C68Ch */ case   26:  		/* bl -11956 */
		/* 8226C68Ch case   26:*/		regs.LR = 0x8226C690; return 0x822697D8;
		/* 8226C68Ch case   26:*/		return 0x8226C690;
		  /* 8226C690h */ case   27:  		/* nor R11, R3, R3 */
		/* 8226C690h case   27:*/		cpu::op::nor<0>(regs,&regs.R11,regs.R3,regs.R3);
		/* 8226C690h case   27:*/		return 0x8226C694;
		  /* 8226C694h */ case   28:  		/* rlwinm R11, R11, 31, 31, 31 */
		/* 8226C694h case   28:*/		cpu::op::rlwinm<0,31,31,31>(regs,&regs.R11,regs.R11);
		/* 8226C694h case   28:*/		return 0x8226C698;
		  /* 8226C698h */ case   29:  		/* b 20 */
		/* 8226C698h case   29:*/		return 0x8226C6AC;
		/* 8226C698h case   29:*/		return 0x8226C69C;
	}
	return 0x8226C69C;
} // Block from 8226C624h-8226C69Ch (30 instructions)

//////////////////////////////////////////////////////
// Block at 8226C69Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C69C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C69C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C69C);
		  /* 8226C69Ch */ case    0:  		/* rlwinm. R11, R11, 0, 8, 11 */
		/* 8226C69Ch case    0:*/		cpu::op::rlwinm<1,0,8,11>(regs,&regs.R11,regs.R11);
		/* 8226C69Ch case    0:*/		return 0x8226C6A0;
		  /* 8226C6A0h */ case    1:  		/* li R11, 1 */
		/* 8226C6A0h case    1:*/		cpu::op::li<0>(regs,&regs.R11,0x1);
		/* 8226C6A0h case    1:*/		return 0x8226C6A4;
		  /* 8226C6A4h */ case    2:  		/* bc 12, CR0_EQ, 8 */
		/* 8226C6A4h case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C6AC;  }
		/* 8226C6A4h case    2:*/		return 0x8226C6A8;
	}
	return 0x8226C6A8;
} // Block from 8226C69Ch-8226C6A8h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C6A8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C6A8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C6A8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C6A8);
		  /* 8226C6A8h */ case    0:  		/* li R11, 0 */
		/* 8226C6A8h case    0:*/		cpu::op::li<0>(regs,&regs.R11,0x0);
		/* 8226C6A8h case    0:*/		return 0x8226C6AC;
	}
	return 0x8226C6AC;
} // Block from 8226C6A8h-8226C6ACh (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226C6ACh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C6AC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C6AC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C6AC);
		  /* 8226C6ACh */ case    0:  		/* rlwinm. R11, R11, 0, 24, 31 */
		/* 8226C6ACh case    0:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R11,regs.R11);
		/* 8226C6ACh case    0:*/		return 0x8226C6B0;
		  /* 8226C6B0h */ case    1:  		/* bc 12, CR0_EQ, 184 */
		/* 8226C6B0h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226C768;  }
		/* 8226C6B0h case    1:*/		return 0x8226C6B4;
	}
	return 0x8226C6B4;
} // Block from 8226C6ACh-8226C6B4h (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C6B4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C6B4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C6B4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C6B4);
		  /* 8226C6B4h */ case    0:  		/* mr R3, R31 */
		/* 8226C6B4h case    0:*/		regs.R3 = regs.R31;
		/* 8226C6B4h case    0:*/		return 0x8226C6B8;
		  /* 8226C6B8h */ case    1:  		/* bl -6648 */
		/* 8226C6B8h case    1:*/		regs.LR = 0x8226C6BC; return 0x8226ACC0;
		/* 8226C6B8h case    1:*/		return 0x8226C6BC;
		  /* 8226C6BCh */ case    2:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226C6BCh case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C6BCh case    2:*/		return 0x8226C6C0;
		  /* 8226C6C0h */ case    3:  		/* rlwinm. R11, R11, 0, 26, 26 */
		/* 8226C6C0h case    3:*/		cpu::op::rlwinm<1,0,26,26>(regs,&regs.R11,regs.R11);
		/* 8226C6C0h case    3:*/		return 0x8226C6C4;
		  /* 8226C6C4h */ case    4:  		/* bc 12, CR0_EQ, 20 */
		/* 8226C6C4h case    4:*/		if ( regs.CR[0].eq ) { return 0x8226C6D8;  }
		/* 8226C6C4h case    4:*/		return 0x8226C6C8;
		  /* 8226C6C8h */ case    5:  		/* mr R4, R29 */
		/* 8226C6C8h case    5:*/		regs.R4 = regs.R29;
		/* 8226C6C8h case    5:*/		return 0x8226C6CC;
		  /* 8226C6CCh */ case    6:  		/* mr R5, R22 */
		/* 8226C6CCh case    6:*/		regs.R5 = regs.R22;
		/* 8226C6CCh case    6:*/		return 0x8226C6D0;
		  /* 8226C6D0h */ case    7:  		/* mr R3, R31 */
		/* 8226C6D0h case    7:*/		regs.R3 = regs.R31;
		/* 8226C6D0h case    7:*/		return 0x8226C6D4;
		  /* 8226C6D4h */ case    8:  		/* bl -11508 */
		/* 8226C6D4h case    8:*/		regs.LR = 0x8226C6D8; return 0x822699E0;
		/* 8226C6D4h case    8:*/		return 0x8226C6D8;
	}
	return 0x8226C6D8;
} // Block from 8226C6B4h-8226C6D8h (9 instructions)

//////////////////////////////////////////////////////
// Block at 8226C6D8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C6D8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C6D8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C6D8);
		  /* 8226C6D8h */ case    0:  		/* rlwinm. R11, R21, 0, 24, 31 */
		/* 8226C6D8h case    0:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R11,regs.R21);
		/* 8226C6D8h case    0:*/		return 0x8226C6DC;
		  /* 8226C6DCh */ case    1:  		/* bc 12, CR0_EQ, 72 */
		/* 8226C6DCh case    1:*/		if ( regs.CR[0].eq ) { return 0x8226C724;  }
		/* 8226C6DCh case    1:*/		return 0x8226C6E0;
		  /* 8226C6E0h */ case    2:  		/* mr R4, R30 */
		/* 8226C6E0h case    2:*/		regs.R4 = regs.R30;
		/* 8226C6E0h case    2:*/		return 0x8226C6E4;
		  /* 8226C6E4h */ case    3:  		/* mr R3, R31 */
		/* 8226C6E4h case    3:*/		regs.R3 = regs.R31;
		/* 8226C6E4h case    3:*/		return 0x8226C6E8;
		  /* 8226C6E8h */ case    4:  		/* bl -11528 */
		/* 8226C6E8h case    4:*/		regs.LR = 0x8226C6EC; return 0x822699E0;
		/* 8226C6E8h case    4:*/		return 0x8226C6EC;
		  /* 8226C6ECh */ case    5:  		/* mr R4, R26 */
		/* 8226C6ECh case    5:*/		regs.R4 = regs.R26;
		/* 8226C6ECh case    5:*/		return 0x8226C6F0;
		  /* 8226C6F0h */ case    6:  		/* mr R3, R31 */
		/* 8226C6F0h case    6:*/		regs.R3 = regs.R31;
		/* 8226C6F0h case    6:*/		return 0x8226C6F4;
		  /* 8226C6F4h */ case    7:  		/* bl -11076 */
		/* 8226C6F4h case    7:*/		regs.LR = 0x8226C6F8; return 0x82269BB0;
		/* 8226C6F4h case    7:*/		return 0x8226C6F8;
		  /* 8226C6F8h */ case    8:  		/* lbz R11, <#[R31 + 8232]> */
		/* 8226C6F8h case    8:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x00002028) );
		/* 8226C6F8h case    8:*/		return 0x8226C6FC;
		  /* 8226C6FCh */ case    9:  		/* cmplwi CR0, R11, 0 */
		/* 8226C6FCh case    9:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C6FCh case    9:*/		return 0x8226C700;
		  /* 8226C700h */ case   10:  		/* bc 4, CR0_EQ, 12 */
		/* 8226C700h case   10:*/		if ( !regs.CR[0].eq ) { return 0x8226C70C;  }
		/* 8226C700h case   10:*/		return 0x8226C704;
		  /* 8226C704h */ case   11:  		/* mr R3, R31 */
		/* 8226C704h case   11:*/		regs.R3 = regs.R31;
		/* 8226C704h case   11:*/		return 0x8226C708;
		  /* 8226C708h */ case   12:  		/* bl -6816 */
		/* 8226C708h case   12:*/		regs.LR = 0x8226C70C; return 0x8226AC68;
		/* 8226C708h case   12:*/		return 0x8226C70C;
	}
	return 0x8226C70C;
} // Block from 8226C6D8h-8226C70Ch (13 instructions)

//////////////////////////////////////////////////////
// Block at 8226C70Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C70C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C70C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C70C);
		  /* 8226C70Ch */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226C70Ch case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C70Ch case    0:*/		return 0x8226C710;
		  /* 8226C710h */ case    1:  		/* rlwinm. R11, R11, 0, 26, 26 */
		/* 8226C710h case    1:*/		cpu::op::rlwinm<1,0,26,26>(regs,&regs.R11,regs.R11);
		/* 8226C710h case    1:*/		return 0x8226C714;
		  /* 8226C714h */ case    2:  		/* bc 12, CR0_EQ, 16 */
		/* 8226C714h case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C724;  }
		/* 8226C714h case    2:*/		return 0x8226C718;
		  /* 8226C718h */ case    3:  		/* mr R4, R25 */
		/* 8226C718h case    3:*/		regs.R4 = regs.R25;
		/* 8226C718h case    3:*/		return 0x8226C71C;
		  /* 8226C71Ch */ case    4:  		/* mr R3, R31 */
		/* 8226C71Ch case    4:*/		regs.R3 = regs.R31;
		/* 8226C71Ch case    4:*/		return 0x8226C720;
		  /* 8226C720h */ case    5:  		/* bl -11584 */
		/* 8226C720h case    5:*/		regs.LR = 0x8226C724; return 0x822699E0;
		/* 8226C720h case    5:*/		return 0x8226C724;
	}
	return 0x8226C724;
} // Block from 8226C70Ch-8226C724h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226C724h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C724( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C724) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C724);
		  /* 8226C724h */ case    0:  		/* mr R4, R30 */
		/* 8226C724h case    0:*/		regs.R4 = regs.R30;
		/* 8226C724h case    0:*/		return 0x8226C728;
		  /* 8226C728h */ case    1:  		/* mr R3, R31 */
		/* 8226C728h case    1:*/		regs.R3 = regs.R31;
		/* 8226C728h case    1:*/		return 0x8226C72C;
		  /* 8226C72Ch */ case    2:  		/* bl -11596 */
		/* 8226C72Ch case    2:*/		regs.LR = 0x8226C730; return 0x822699E0;
		/* 8226C72Ch case    2:*/		return 0x8226C730;
		  /* 8226C730h */ case    3:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C730h case    3:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C730h case    3:*/		return 0x8226C734;
		  /* 8226C734h */ case    4:  		/* rlwinm R11, R11, 5, 27, 31 */
		/* 8226C734h case    4:*/		cpu::op::rlwinm<0,5,27,31>(regs,&regs.R11,regs.R11);
		/* 8226C734h case    4:*/		return 0x8226C738;
		  /* 8226C738h */ case    5:  		/* rlwinm R10, R11, 0, 30, 31 */
		/* 8226C738h case    5:*/		cpu::op::rlwinm<0,0,30,31>(regs,&regs.R10,regs.R11);
		/* 8226C738h case    5:*/		return 0x8226C73C;
		  /* 8226C73Ch */ case    6:  		/* cmplwi CR6, R10, 2 */
		/* 8226C73Ch case    6:*/		cpu::op::cmplwi<6>(regs,regs.R10,0x00000002);
		/* 8226C73Ch case    6:*/		return 0x8226C740;
		  /* 8226C740h */ case    7:  		/* bc 12, CR6_LT, 28 */
		/* 8226C740h case    7:*/		if ( regs.CR[6].lt ) { return 0x8226C75C;  }
		/* 8226C740h case    7:*/		return 0x8226C744;
		  /* 8226C744h */ case    8:  		/* rlwinm. R11, R11, 0, 31, 31 */
		/* 8226C744h case    8:*/		cpu::op::rlwinm<1,0,31,31>(regs,&regs.R11,regs.R11);
		/* 8226C744h case    8:*/		return 0x8226C748;
		  /* 8226C748h */ case    9:  		/* mr R4, R23 */
		/* 8226C748h case    9:*/		regs.R4 = regs.R23;
		/* 8226C748h case    9:*/		return 0x8226C74C;
		  /* 8226C74Ch */ case   10:  		/* bc 4, CR0_EQ, 8 */
		/* 8226C74Ch case   10:*/		if ( !regs.CR[0].eq ) { return 0x8226C754;  }
		/* 8226C74Ch case   10:*/		return 0x8226C750;
		  /* 8226C750h */ case   11:  		/* mr R4, R24 */
		/* 8226C750h case   11:*/		regs.R4 = regs.R24;
		/* 8226C750h case   11:*/		return 0x8226C754;
	}
	return 0x8226C754;
} // Block from 8226C724h-8226C754h (12 instructions)

//////////////////////////////////////////////////////
// Block at 8226C754h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C754( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C754) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C754);
		  /* 8226C754h */ case    0:  		/* mr R3, R31 */
		/* 8226C754h case    0:*/		regs.R3 = regs.R31;
		/* 8226C754h case    0:*/		return 0x8226C758;
		  /* 8226C758h */ case    1:  		/* bl -11640 */
		/* 8226C758h case    1:*/		regs.LR = 0x8226C75C; return 0x822699E0;
		/* 8226C758h case    1:*/		return 0x8226C75C;
	}
	return 0x8226C75C;
} // Block from 8226C754h-8226C75Ch (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C75Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C75C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C75C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C75C);
		  /* 8226C75Ch */ case    0:  		/* mr R4, R28 */
		/* 8226C75Ch case    0:*/		regs.R4 = regs.R28;
		/* 8226C75Ch case    0:*/		return 0x8226C760;
		  /* 8226C760h */ case    1:  		/* mr R3, R31 */
		/* 8226C760h case    1:*/		regs.R3 = regs.R31;
		/* 8226C760h case    1:*/		return 0x8226C764;
		  /* 8226C764h */ case    2:  		/* bl -8996 */
		/* 8226C764h case    2:*/		regs.LR = 0x8226C768; return 0x8226A440;
		/* 8226C764h case    2:*/		return 0x8226C768;
	}
	return 0x8226C768;
} // Block from 8226C75Ch-8226C768h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C768h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C768( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C768) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C768);
		  /* 8226C768h */ case    0:  		/* lwz R11, <#[R28]> */
		/* 8226C768h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000000) );
		/* 8226C768h case    0:*/		return 0x8226C76C;
		  /* 8226C76Ch */ case    1:  		/* rlwinm R10, R11, 0, 0, 5 */
		/* 8226C76Ch case    1:*/		cpu::op::rlwinm<0,0,0,5>(regs,&regs.R10,regs.R11);
		/* 8226C76Ch case    1:*/		return 0x8226C770;
		  /* 8226C770h */ case    2:  		/* cmplw CR6, R10, R27 */
		/* 8226C770h case    2:*/		cpu::op::cmplw<6>(regs,regs.R10,regs.R27);
		/* 8226C770h case    2:*/		return 0x8226C774;
		  /* 8226C774h */ case    3:  		/* bc 4, CR6_EQ, 44 */
		/* 8226C774h case    3:*/		if ( !regs.CR[6].eq ) { return 0x8226C7A0;  }
		/* 8226C774h case    3:*/		return 0x8226C778;
		  /* 8226C778h */ case    4:  		/* rlwinm. R10, R11, 0, 16, 16 */
		/* 8226C778h case    4:*/		cpu::op::rlwinm<1,0,16,16>(regs,&regs.R10,regs.R11);
		/* 8226C778h case    4:*/		return 0x8226C77C;
		  /* 8226C77Ch */ case    5:  		/* bc 12, CR0_EQ, 24 */
		/* 8226C77Ch case    5:*/		if ( regs.CR[0].eq ) { return 0x8226C794;  }
		/* 8226C77Ch case    5:*/		return 0x8226C780;
		  /* 8226C780h */ case    6:  		/* mr R3, R28 */
		/* 8226C780h case    6:*/		regs.R3 = regs.R28;
		/* 8226C780h case    6:*/		return 0x8226C784;
		  /* 8226C784h */ case    7:  		/* bl -12204 */
		/* 8226C784h case    7:*/		regs.LR = 0x8226C788; return 0x822697D8;
		/* 8226C784h case    7:*/		return 0x8226C788;
		  /* 8226C788h */ case    8:  		/* nor R11, R3, R3 */
		/* 8226C788h case    8:*/		cpu::op::nor<0>(regs,&regs.R11,regs.R3,regs.R3);
		/* 8226C788h case    8:*/		return 0x8226C78C;
		  /* 8226C78Ch */ case    9:  		/* rlwinm R11, R11, 31, 31, 31 */
		/* 8226C78Ch case    9:*/		cpu::op::rlwinm<0,31,31,31>(regs,&regs.R11,regs.R11);
		/* 8226C78Ch case    9:*/		return 0x8226C790;
		  /* 8226C790h */ case   10:  		/* b 20 */
		/* 8226C790h case   10:*/		return 0x8226C7A4;
		/* 8226C790h case   10:*/		return 0x8226C794;
	}
	return 0x8226C794;
} // Block from 8226C768h-8226C794h (11 instructions)

//////////////////////////////////////////////////////
// Block at 8226C794h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C794( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C794) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C794);
		  /* 8226C794h */ case    0:  		/* rlwinm. R11, R11, 0, 8, 11 */
		/* 8226C794h case    0:*/		cpu::op::rlwinm<1,0,8,11>(regs,&regs.R11,regs.R11);
		/* 8226C794h case    0:*/		return 0x8226C798;
		  /* 8226C798h */ case    1:  		/* li R11, 1 */
		/* 8226C798h case    1:*/		cpu::op::li<0>(regs,&regs.R11,0x1);
		/* 8226C798h case    1:*/		return 0x8226C79C;
		  /* 8226C79Ch */ case    2:  		/* bc 12, CR0_EQ, 8 */
		/* 8226C79Ch case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C7A4;  }
		/* 8226C79Ch case    2:*/		return 0x8226C7A0;
	}
	return 0x8226C7A0;
} // Block from 8226C794h-8226C7A0h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C7A0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C7A0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C7A0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C7A0);
		  /* 8226C7A0h */ case    0:  		/* li R11, 0 */
		/* 8226C7A0h case    0:*/		cpu::op::li<0>(regs,&regs.R11,0x0);
		/* 8226C7A0h case    0:*/		return 0x8226C7A4;
	}
	return 0x8226C7A4;
} // Block from 8226C7A0h-8226C7A4h (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226C7A4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C7A4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C7A4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C7A4);
		  /* 8226C7A4h */ case    0:  		/* rlwinm. R11, R11, 0, 24, 31 */
		/* 8226C7A4h case    0:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R11,regs.R11);
		/* 8226C7A4h case    0:*/		return 0x8226C7A8;
		  /* 8226C7A8h */ case    1:  		/* bc 4, CR0_EQ, 256 */
		/* 8226C7A8h case    1:*/		if ( !regs.CR[0].eq ) { return 0x8226C8A8;  }
		/* 8226C7A8h case    1:*/		return 0x8226C7AC;
		  /* 8226C7ACh */ case    2:  		/* mr R3, R28 */
		/* 8226C7ACh case    2:*/		regs.R3 = regs.R28;
		/* 8226C7ACh case    2:*/		return 0x8226C7B0;
		  /* 8226C7B0h */ case    3:  		/* bl -12096 */
		/* 8226C7B0h case    3:*/		regs.LR = 0x8226C7B4; return 0x82269870;
		/* 8226C7B0h case    3:*/		return 0x8226C7B4;
		  /* 8226C7B4h */ case    4:  		/* rlwinm. R11, R3, 0, 24, 31 */
		/* 8226C7B4h case    4:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R11,regs.R3);
		/* 8226C7B4h case    4:*/		return 0x8226C7B8;
		  /* 8226C7B8h */ case    5:  		/* bc 4, CR0_EQ, 60 */
		/* 8226C7B8h case    5:*/		if ( !regs.CR[0].eq ) { return 0x8226C7F4;  }
		/* 8226C7B8h case    5:*/		return 0x8226C7BC;
		  /* 8226C7BCh */ case    6:  		/* lbz R11, <#[R31 + 8232]> */
		/* 8226C7BCh case    6:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x00002028) );
		/* 8226C7BCh case    6:*/		return 0x8226C7C0;
		  /* 8226C7C0h */ case    7:  		/* cmplwi CR0, R11, 0 */
		/* 8226C7C0h case    7:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C7C0h case    7:*/		return 0x8226C7C4;
		  /* 8226C7C4h */ case    8:  		/* bc 4, CR0_EQ, 12 */
		/* 8226C7C4h case    8:*/		if ( !regs.CR[0].eq ) { return 0x8226C7D0;  }
		/* 8226C7C4h case    8:*/		return 0x8226C7C8;
		  /* 8226C7C8h */ case    9:  		/* mr R3, R31 */
		/* 8226C7C8h case    9:*/		regs.R3 = regs.R31;
		/* 8226C7C8h case    9:*/		return 0x8226C7CC;
		  /* 8226C7CCh */ case   10:  		/* bl -7012 */
		/* 8226C7CCh case   10:*/		regs.LR = 0x8226C7D0; return 0x8226AC68;
		/* 8226C7CCh case   10:*/		return 0x8226C7D0;
	}
	return 0x8226C7D0;
} // Block from 8226C7A4h-8226C7D0h (11 instructions)

//////////////////////////////////////////////////////
// Block at 8226C7D0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C7D0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C7D0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C7D0);
		  /* 8226C7D0h */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226C7D0h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C7D0h case    0:*/		return 0x8226C7D4;
		  /* 8226C7D4h */ case    1:  		/* rlwinm. R11, R11, 0, 26, 26 */
		/* 8226C7D4h case    1:*/		cpu::op::rlwinm<1,0,26,26>(regs,&regs.R11,regs.R11);
		/* 8226C7D4h case    1:*/		return 0x8226C7D8;
		  /* 8226C7D8h */ case    2:  		/* bc 12, CR0_EQ, 16 */
		/* 8226C7D8h case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C7E8;  }
		/* 8226C7D8h case    2:*/		return 0x8226C7DC;
		  /* 8226C7DCh */ case    3:  		/* mr R4, R25 */
		/* 8226C7DCh case    3:*/		regs.R4 = regs.R25;
		/* 8226C7DCh case    3:*/		return 0x8226C7E0;
		  /* 8226C7E0h */ case    4:  		/* mr R3, R31 */
		/* 8226C7E0h case    4:*/		regs.R3 = regs.R31;
		/* 8226C7E0h case    4:*/		return 0x8226C7E4;
		  /* 8226C7E4h */ case    5:  		/* bl -11780 */
		/* 8226C7E4h case    5:*/		regs.LR = 0x8226C7E8; return 0x822699E0;
		/* 8226C7E4h case    5:*/		return 0x8226C7E8;
	}
	return 0x8226C7E8;
} // Block from 8226C7D0h-8226C7E8h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226C7E8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C7E8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C7E8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C7E8);
		  /* 8226C7E8h */ case    0:  		/* lis R11, -32252 */
		/* 8226C7E8h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8204);
		/* 8226C7E8h case    0:*/		return 0x8226C7EC;
		  /* 8226C7ECh */ case    1:  		/* addi R4, R11, -22696 */
		/* 8226C7ECh case    1:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFA758);
		/* 8226C7ECh case    1:*/		return 0x8226C7F0;
		  /* 8226C7F0h */ case    2:  		/* b 120 */
		/* 8226C7F0h case    2:*/		return 0x8226C868;
		/* 8226C7F0h case    2:*/		return 0x8226C7F4;
	}
	return 0x8226C7F4;
} // Block from 8226C7E8h-8226C7F4h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C7F4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C7F4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C7F4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C7F4);
		  /* 8226C7F4h */ case    0:  		/* mr R3, R31 */
		/* 8226C7F4h case    0:*/		regs.R3 = regs.R31;
		/* 8226C7F4h case    0:*/		return 0x8226C7F8;
		  /* 8226C7F8h */ case    1:  		/* bl -6968 */
		/* 8226C7F8h case    1:*/		regs.LR = 0x8226C7FC; return 0x8226ACC0;
		/* 8226C7F8h case    1:*/		return 0x8226C7FC;
		  /* 8226C7FCh */ case    2:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226C7FCh case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C7FCh case    2:*/		return 0x8226C800;
		  /* 8226C800h */ case    3:  		/* rlwinm. R11, R11, 0, 26, 26 */
		/* 8226C800h case    3:*/		cpu::op::rlwinm<1,0,26,26>(regs,&regs.R11,regs.R11);
		/* 8226C800h case    3:*/		return 0x8226C804;
		  /* 8226C804h */ case    4:  		/* bc 12, CR0_EQ, 20 */
		/* 8226C804h case    4:*/		if ( regs.CR[0].eq ) { return 0x8226C818;  }
		/* 8226C804h case    4:*/		return 0x8226C808;
		  /* 8226C808h */ case    5:  		/* mr R4, R29 */
		/* 8226C808h case    5:*/		regs.R4 = regs.R29;
		/* 8226C808h case    5:*/		return 0x8226C80C;
		  /* 8226C80Ch */ case    6:  		/* mr R5, R22 */
		/* 8226C80Ch case    6:*/		regs.R5 = regs.R22;
		/* 8226C80Ch case    6:*/		return 0x8226C810;
		  /* 8226C810h */ case    7:  		/* mr R3, R31 */
		/* 8226C810h case    7:*/		regs.R3 = regs.R31;
		/* 8226C810h case    7:*/		return 0x8226C814;
		  /* 8226C814h */ case    8:  		/* bl -11828 */
		/* 8226C814h case    8:*/		regs.LR = 0x8226C818; return 0x822699E0;
		/* 8226C814h case    8:*/		return 0x8226C818;
	}
	return 0x8226C818;
} // Block from 8226C7F4h-8226C818h (9 instructions)

//////////////////////////////////////////////////////
// Block at 8226C818h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C818( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C818) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C818);
		  /* 8226C818h */ case    0:  		/* rlwinm. R11, R21, 0, 24, 31 */
		/* 8226C818h case    0:*/		cpu::op::rlwinm<1,0,24,31>(regs,&regs.R11,regs.R21);
		/* 8226C818h case    0:*/		return 0x8226C81C;
		  /* 8226C81Ch */ case    1:  		/* bc 12, CR0_EQ, 72 */
		/* 8226C81Ch case    1:*/		if ( regs.CR[0].eq ) { return 0x8226C864;  }
		/* 8226C81Ch case    1:*/		return 0x8226C820;
		  /* 8226C820h */ case    2:  		/* mr R4, R30 */
		/* 8226C820h case    2:*/		regs.R4 = regs.R30;
		/* 8226C820h case    2:*/		return 0x8226C824;
		  /* 8226C824h */ case    3:  		/* mr R3, R31 */
		/* 8226C824h case    3:*/		regs.R3 = regs.R31;
		/* 8226C824h case    3:*/		return 0x8226C828;
		  /* 8226C828h */ case    4:  		/* bl -11848 */
		/* 8226C828h case    4:*/		regs.LR = 0x8226C82C; return 0x822699E0;
		/* 8226C828h case    4:*/		return 0x8226C82C;
		  /* 8226C82Ch */ case    5:  		/* mr R4, R26 */
		/* 8226C82Ch case    5:*/		regs.R4 = regs.R26;
		/* 8226C82Ch case    5:*/		return 0x8226C830;
		  /* 8226C830h */ case    6:  		/* mr R3, R31 */
		/* 8226C830h case    6:*/		regs.R3 = regs.R31;
		/* 8226C830h case    6:*/		return 0x8226C834;
		  /* 8226C834h */ case    7:  		/* bl -11396 */
		/* 8226C834h case    7:*/		regs.LR = 0x8226C838; return 0x82269BB0;
		/* 8226C834h case    7:*/		return 0x8226C838;
		  /* 8226C838h */ case    8:  		/* lbz R11, <#[R31 + 8232]> */
		/* 8226C838h case    8:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x00002028) );
		/* 8226C838h case    8:*/		return 0x8226C83C;
		  /* 8226C83Ch */ case    9:  		/* cmplwi CR0, R11, 0 */
		/* 8226C83Ch case    9:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C83Ch case    9:*/		return 0x8226C840;
		  /* 8226C840h */ case   10:  		/* bc 4, CR0_EQ, 12 */
		/* 8226C840h case   10:*/		if ( !regs.CR[0].eq ) { return 0x8226C84C;  }
		/* 8226C840h case   10:*/		return 0x8226C844;
		  /* 8226C844h */ case   11:  		/* mr R3, R31 */
		/* 8226C844h case   11:*/		regs.R3 = regs.R31;
		/* 8226C844h case   11:*/		return 0x8226C848;
		  /* 8226C848h */ case   12:  		/* bl -7136 */
		/* 8226C848h case   12:*/		regs.LR = 0x8226C84C; return 0x8226AC68;
		/* 8226C848h case   12:*/		return 0x8226C84C;
	}
	return 0x8226C84C;
} // Block from 8226C818h-8226C84Ch (13 instructions)

//////////////////////////////////////////////////////
// Block at 8226C84Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C84C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C84C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C84C);
		  /* 8226C84Ch */ case    0:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226C84Ch case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226C84Ch case    0:*/		return 0x8226C850;
		  /* 8226C850h */ case    1:  		/* rlwinm. R11, R11, 0, 26, 26 */
		/* 8226C850h case    1:*/		cpu::op::rlwinm<1,0,26,26>(regs,&regs.R11,regs.R11);
		/* 8226C850h case    1:*/		return 0x8226C854;
		  /* 8226C854h */ case    2:  		/* bc 12, CR0_EQ, 16 */
		/* 8226C854h case    2:*/		if ( regs.CR[0].eq ) { return 0x8226C864;  }
		/* 8226C854h case    2:*/		return 0x8226C858;
		  /* 8226C858h */ case    3:  		/* mr R4, R25 */
		/* 8226C858h case    3:*/		regs.R4 = regs.R25;
		/* 8226C858h case    3:*/		return 0x8226C85C;
		  /* 8226C85Ch */ case    4:  		/* mr R3, R31 */
		/* 8226C85Ch case    4:*/		regs.R3 = regs.R31;
		/* 8226C85Ch case    4:*/		return 0x8226C860;
		  /* 8226C860h */ case    5:  		/* bl -11904 */
		/* 8226C860h case    5:*/		regs.LR = 0x8226C864; return 0x822699E0;
		/* 8226C860h case    5:*/		return 0x8226C864;
	}
	return 0x8226C864;
} // Block from 8226C84Ch-8226C864h (6 instructions)

//////////////////////////////////////////////////////
// Block at 8226C864h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C864( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C864) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C864);
		  /* 8226C864h */ case    0:  		/* mr R4, R30 */
		/* 8226C864h case    0:*/		regs.R4 = regs.R30;
		/* 8226C864h case    0:*/		return 0x8226C868;
	}
	return 0x8226C868;
} // Block from 8226C864h-8226C868h (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226C868h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C868( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C868) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C868);
		  /* 8226C868h */ case    0:  		/* mr R3, R31 */
		/* 8226C868h case    0:*/		regs.R3 = regs.R31;
		/* 8226C868h case    0:*/		return 0x8226C86C;
		  /* 8226C86Ch */ case    1:  		/* bl -11916 */
		/* 8226C86Ch case    1:*/		regs.LR = 0x8226C870; return 0x822699E0;
		/* 8226C86Ch case    1:*/		return 0x8226C870;
		  /* 8226C870h */ case    2:  		/* lwz R11, <#[R28 + 4]> */
		/* 8226C870h case    2:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R28 + 0x00000004) );
		/* 8226C870h case    2:*/		return 0x8226C874;
		  /* 8226C874h */ case    3:  		/* rlwinm R11, R11, 5, 27, 31 */
		/* 8226C874h case    3:*/		cpu::op::rlwinm<0,5,27,31>(regs,&regs.R11,regs.R11);
		/* 8226C874h case    3:*/		return 0x8226C878;
		  /* 8226C878h */ case    4:  		/* rlwinm R10, R11, 0, 30, 31 */
		/* 8226C878h case    4:*/		cpu::op::rlwinm<0,0,30,31>(regs,&regs.R10,regs.R11);
		/* 8226C878h case    4:*/		return 0x8226C87C;
		  /* 8226C87Ch */ case    5:  		/* cmplwi CR6, R10, 2 */
		/* 8226C87Ch case    5:*/		cpu::op::cmplwi<6>(regs,regs.R10,0x00000002);
		/* 8226C87Ch case    5:*/		return 0x8226C880;
		  /* 8226C880h */ case    6:  		/* bc 12, CR6_LT, 28 */
		/* 8226C880h case    6:*/		if ( regs.CR[6].lt ) { return 0x8226C89C;  }
		/* 8226C880h case    6:*/		return 0x8226C884;
		  /* 8226C884h */ case    7:  		/* rlwinm. R11, R11, 0, 31, 31 */
		/* 8226C884h case    7:*/		cpu::op::rlwinm<1,0,31,31>(regs,&regs.R11,regs.R11);
		/* 8226C884h case    7:*/		return 0x8226C888;
		  /* 8226C888h */ case    8:  		/* mr R4, R23 */
		/* 8226C888h case    8:*/		regs.R4 = regs.R23;
		/* 8226C888h case    8:*/		return 0x8226C88C;
		  /* 8226C88Ch */ case    9:  		/* bc 4, CR0_EQ, 8 */
		/* 8226C88Ch case    9:*/		if ( !regs.CR[0].eq ) { return 0x8226C894;  }
		/* 8226C88Ch case    9:*/		return 0x8226C890;
		  /* 8226C890h */ case   10:  		/* mr R4, R24 */
		/* 8226C890h case   10:*/		regs.R4 = regs.R24;
		/* 8226C890h case   10:*/		return 0x8226C894;
	}
	return 0x8226C894;
} // Block from 8226C868h-8226C894h (11 instructions)

//////////////////////////////////////////////////////
// Block at 8226C894h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C894( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C894) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C894);
		  /* 8226C894h */ case    0:  		/* mr R3, R31 */
		/* 8226C894h case    0:*/		regs.R3 = regs.R31;
		/* 8226C894h case    0:*/		return 0x8226C898;
		  /* 8226C898h */ case    1:  		/* bl -11960 */
		/* 8226C898h case    1:*/		regs.LR = 0x8226C89C; return 0x822699E0;
		/* 8226C898h case    1:*/		return 0x8226C89C;
	}
	return 0x8226C89C;
} // Block from 8226C894h-8226C89Ch (2 instructions)

//////////////////////////////////////////////////////
// Block at 8226C89Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C89C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C89C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C89C);
		  /* 8226C89Ch */ case    0:  		/* mr R4, R28 */
		/* 8226C89Ch case    0:*/		regs.R4 = regs.R28;
		/* 8226C89Ch case    0:*/		return 0x8226C8A0;
		  /* 8226C8A0h */ case    1:  		/* mr R3, R31 */
		/* 8226C8A0h case    1:*/		regs.R3 = regs.R31;
		/* 8226C8A0h case    1:*/		return 0x8226C8A4;
		  /* 8226C8A4h */ case    2:  		/* bl -8324 */
		/* 8226C8A4h case    2:*/		regs.LR = 0x8226C8A8; return 0x8226A820;
		/* 8226C8A4h case    2:*/		return 0x8226C8A8;
	}
	return 0x8226C8A8;
} // Block from 8226C89Ch-8226C8A8h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C8A8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C8A8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C8A8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C8A8);
		  /* 8226C8A8h */ case    0:  		/* lbz R11, <#[R31 + 8232]> */
		/* 8226C8A8h case    0:*/		cpu::mem::load8z( regs, &regs.R11, (uint32)(regs.R31 + 0x00002028) );
		/* 8226C8A8h case    0:*/		return 0x8226C8AC;
		  /* 8226C8ACh */ case    1:  		/* cmplwi CR0, R11, 0 */
		/* 8226C8ACh case    1:*/		cpu::op::cmplwi<0>(regs,regs.R11,0x00000000);
		/* 8226C8ACh case    1:*/		return 0x8226C8B0;
		  /* 8226C8B0h */ case    2:  		/* bc 4, CR0_EQ, 12 */
		/* 8226C8B0h case    2:*/		if ( !regs.CR[0].eq ) { return 0x8226C8BC;  }
		/* 8226C8B0h case    2:*/		return 0x8226C8B4;
		  /* 8226C8B4h */ case    3:  		/* mr R3, R31 */
		/* 8226C8B4h case    3:*/		regs.R3 = regs.R31;
		/* 8226C8B4h case    3:*/		return 0x8226C8B8;
		  /* 8226C8B8h */ case    4:  		/* bl -7248 */
		/* 8226C8B8h case    4:*/		regs.LR = 0x8226C8BC; return 0x8226AC68;
		/* 8226C8B8h case    4:*/		return 0x8226C8BC;
	}
	return 0x8226C8BC;
} // Block from 8226C8A8h-8226C8BCh (5 instructions)

//////////////////////////////////////////////////////
// Block at 8226C8BCh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C8BC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C8BC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C8BC);
		  /* 8226C8BCh */ case    0:  		/* lwz R3, <#[R31 + 8236]> */
		/* 8226C8BCh case    0:*/		cpu::mem::load32z( regs, &regs.R3, (uint32)(regs.R31 + 0x0000202C) );
		/* 8226C8BCh case    0:*/		return 0x8226C8C0;
		  /* 8226C8C0h */ case    1:  		/* addi R1, R1, 208 */
		/* 8226C8C0h case    1:*/		cpu::op::addi<0>(regs,&regs.R1,regs.R1,0xD0);
		/* 8226C8C0h case    1:*/		return 0x8226C8C4;
		  /* 8226C8C4h */ case    2:  		/* b -1947192 */
		/* 8226C8C4h case    2:*/		return 0x8209128C;
		/* 8226C8C4h case    2:*/		return 0x8226C8C8;
	}
	return 0x8226C8C8;
} // Block from 8226C8BCh-8226C8C8h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226C8C8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C8C8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C8C8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C8C8);
		  /* 8226C8C8h */ case    0:  		/* mfspr R12, LR */
		/* 8226C8C8h case    0:*/		regs.R12 = regs.LR;
		/* 8226C8C8h case    0:*/		return 0x8226C8CC;
		  /* 8226C8CCh */ case    1:  		/* stw R12, <#[R1 - 8]> */
		/* 8226C8CCh case    1:*/		cpu::mem::store32( regs, regs.R12, (uint32)(regs.R1 + 0xFFFFFFF8) );
		/* 8226C8CCh case    1:*/		return 0x8226C8D0;
		  /* 8226C8D0h */ case    2:  		/* std R30, <#[R1 - 24]> */
		/* 8226C8D0h case    2:*/		cpu::mem::store64( regs, regs.R30, (uint32)(regs.R1 + 0xFFFFFFE8) );
		/* 8226C8D0h case    2:*/		return 0x8226C8D4;
		  /* 8226C8D4h */ case    3:  		/* std R31, <#[R1 - 16]> */
		/* 8226C8D4h case    3:*/		cpu::mem::store64( regs, regs.R31, (uint32)(regs.R1 + 0xFFFFFFF0) );
		/* 8226C8D4h case    3:*/		return 0x8226C8D8;
		  /* 8226C8D8h */ case    4:  		/* stwu R1, <#[R1 - 144]> */
		/* 8226C8D8h case    4:*/		cpu::mem::store32( regs, regs.R1, (uint32)(regs.R1 + 0xFFFFFF70) );
		regs.R1 = (uint32)(regs.R1 + 0xFFFFFF70);
		/* 8226C8D8h case    4:*/		return 0x8226C8DC;
		  /* 8226C8DCh */ case    5:  		/* mr R30, R3 */
		/* 8226C8DCh case    5:*/		regs.R30 = regs.R3;
		/* 8226C8DCh case    5:*/		return 0x8226C8E0;
		  /* 8226C8E0h */ case    6:  		/* mr R31, R6 */
		/* 8226C8E0h case    6:*/		regs.R31 = regs.R6;
		/* 8226C8E0h case    6:*/		return 0x8226C8E4;
		  /* 8226C8E4h */ case    7:  		/* rlwinm. R11, R4, 0, 31, 31 */
		/* 8226C8E4h case    7:*/		cpu::op::rlwinm<1,0,31,31>(regs,&regs.R11,regs.R4);
		/* 8226C8E4h case    7:*/		return 0x8226C8E8;
		  /* 8226C8E8h */ case    8:  		/* bc 12, CR0_EQ, 112 */
		/* 8226C8E8h case    8:*/		if ( regs.CR[0].eq ) { return 0x8226C958;  }
		/* 8226C8E8h case    8:*/		return 0x8226C8EC;
		  /* 8226C8ECh */ case    9:  		/* lwz R11, <#[R6 + 4]> */
		/* 8226C8ECh case    9:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R6 + 0x00000004) );
		/* 8226C8ECh case    9:*/		return 0x8226C8F0;
		  /* 8226C8F0h */ case   10:  		/* rlwinm. R11, R11, 0, 20, 20 */
		/* 8226C8F0h case   10:*/		cpu::op::rlwinm<1,0,20,20>(regs,&regs.R11,regs.R11);
		/* 8226C8F0h case   10:*/		return 0x8226C8F4;
		  /* 8226C8F4h */ case   11:  		/* bc 12, CR0_EQ, 20 */
		/* 8226C8F4h case   11:*/		if ( regs.CR[0].eq ) { return 0x8226C908;  }
		/* 8226C8F4h case   11:*/		return 0x8226C8F8;
		  /* 8226C8F8h */ case   12:  		/* lis R11, -32251 */
		/* 8226C8F8h case   12:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226C8F8h case   12:*/		return 0x8226C8FC;
		  /* 8226C8FCh */ case   13:  		/* li R4, 0 */
		/* 8226C8FCh case   13:*/		cpu::op::li<0>(regs,&regs.R4,0x0);
		/* 8226C8FCh case   13:*/		return 0x8226C900;
		  /* 8226C900h */ case   14:  		/* addi R6, R11, -15812 */
		/* 8226C900h case   14:*/		cpu::op::addi<0>(regs,&regs.R6,regs.R11,0xFFFFC23C);
		/* 8226C900h case   14:*/		return 0x8226C904;
		  /* 8226C904h */ case   15:  		/* bl -7052 */
		/* 8226C904h case   15:*/		regs.LR = 0x8226C908; return 0x8226AD78;
		/* 8226C904h case   15:*/		return 0x8226C908;
	}
	return 0x8226C908;
} // Block from 8226C8C8h-8226C908h (16 instructions)

//////////////////////////////////////////////////////
// Block at 8226C908h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C908( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C908) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C908);
		  /* 8226C908h */ case    0:  		/* lwz R11, <#[R31 + 4]> */
		/* 8226C908h case    0:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000004) );
		/* 8226C908h case    0:*/		return 0x8226C90C;
		  /* 8226C90Ch */ case    1:  		/* rlwinm R11, R11, 20, 28, 31 */
		/* 8226C90Ch case    1:*/		cpu::op::rlwinm<0,20,28,31>(regs,&regs.R11,regs.R11);
		/* 8226C90Ch case    1:*/		return 0x8226C910;
		  /* 8226C910h */ case    2:  		/* cmplwi CR6, R11, 7 */
		/* 8226C910h case    2:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x00000007);
		/* 8226C910h case    2:*/		return 0x8226C914;
		  /* 8226C914h */ case    3:  		/* bc 12, CR6_EQ, 28 */
		/* 8226C914h case    3:*/		if ( regs.CR[6].eq ) { return 0x8226C930;  }
		/* 8226C914h case    3:*/		return 0x8226C918;
		  /* 8226C918h */ case    4:  		/* cmplwi CR6, R11, 8 */
		/* 8226C918h case    4:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x00000008);
		/* 8226C918h case    4:*/		return 0x8226C91C;
		  /* 8226C91Ch */ case    5:  		/* bc 12, CR6_EQ, 20 */
		/* 8226C91Ch case    5:*/		if ( regs.CR[6].eq ) { return 0x8226C930;  }
		/* 8226C91Ch case    5:*/		return 0x8226C920;
		  /* 8226C920h */ case    6:  		/* cmplwi CR6, R11, 9 */
		/* 8226C920h case    6:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x00000009);
		/* 8226C920h case    6:*/		return 0x8226C924;
		  /* 8226C924h */ case    7:  		/* bc 12, CR6_EQ, 12 */
		/* 8226C924h case    7:*/		if ( regs.CR[6].eq ) { return 0x8226C930;  }
		/* 8226C924h case    7:*/		return 0x8226C928;
		  /* 8226C928h */ case    8:  		/* cmplwi CR6, R11, 11 */
		/* 8226C928h case    8:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x0000000B);
		/* 8226C928h case    8:*/		return 0x8226C92C;
		  /* 8226C92Ch */ case    9:  		/* bc 4, CR6_EQ, 44 */
		/* 8226C92Ch case    9:*/		if ( !regs.CR[6].eq ) { return 0x8226C958;  }
		/* 8226C92Ch case    9:*/		return 0x8226C930;
	}
	return 0x8226C930;
} // Block from 8226C908h-8226C930h (10 instructions)

//////////////////////////////////////////////////////
// Block at 8226C930h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C930( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C930) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C930);
		  /* 8226C930h */ case    0:  		/* lwz R10, <#[R31]> */
		/* 8226C930h case    0:*/		cpu::mem::load32z( regs, &regs.R10, (uint32)(regs.R31 + 0x00000000) );
		/* 8226C930h case    0:*/		return 0x8226C934;
		  /* 8226C934h */ case    1:  		/* addi R11, R30, 24 */
		/* 8226C934h case    1:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R30,0x18);
		/* 8226C934h case    1:*/		return 0x8226C938;
		  /* 8226C938h */ case    2:  		/* li R9, 1 */
		/* 8226C938h case    2:*/		cpu::op::li<0>(regs,&regs.R9,0x1);
		/* 8226C938h case    2:*/		return 0x8226C93C;
		  /* 8226C93Ch */ case    3:  		/* rlwinm R8, R10, 0, 19, 31 */
		/* 8226C93Ch case    3:*/		cpu::op::rlwinm<0,0,19,31>(regs,&regs.R8,regs.R10);
		/* 8226C93Ch case    3:*/		return 0x8226C940;
		  /* 8226C940h */ case    4:  		/* rlwinm R10, R8, 29, 3, 29 */
		/* 8226C940h case    4:*/		cpu::op::rlwinm<0,29,3,29>(regs,&regs.R10,regs.R8);
		/* 8226C940h case    4:*/		return 0x8226C944;
		  /* 8226C944h */ case    5:  		/* rlwinm R8, R8, 0, 27, 31 */
		/* 8226C944h case    5:*/		cpu::op::rlwinm<0,0,27,31>(regs,&regs.R8,regs.R8);
		/* 8226C944h case    5:*/		return 0x8226C948;
		  /* 8226C948h */ case    6:  		/* slw R9, R9, R8 */
		/* 8226C948h case    6:*/		cpu::op::slw<0>(regs,&regs.R9,regs.R9,regs.R8);
		/* 8226C948h case    6:*/		return 0x8226C94C;
		  /* 8226C94Ch */ case    7:  		/* lwzx R8, <#[R10 + R11]> */
		/* 8226C94Ch case    7:*/		cpu::mem::load32z( regs, &regs.R8, (uint32)(regs.R10 + regs.R11 + 0x00000000) );
		/* 8226C94Ch case    7:*/		return 0x8226C950;
		  /* 8226C950h */ case    8:  		/* or R9, R9, R8 */
		/* 8226C950h case    8:*/		cpu::op::or<0>(regs,&regs.R9,regs.R9,regs.R8);
		/* 8226C950h case    8:*/		return 0x8226C954;
		  /* 8226C954h */ case    9:  		/* stwx R9, <#[R10 + R11]> */
		/* 8226C954h case    9:*/		cpu::mem::store32( regs, regs.R9, (uint32)(regs.R10 + regs.R11 + 0x00000000) );
		/* 8226C954h case    9:*/		return 0x8226C958;
	}
	return 0x8226C958;
} // Block from 8226C930h-8226C958h (10 instructions)

//////////////////////////////////////////////////////
// Block at 8226C958h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C958( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C958) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C958);
		  /* 8226C958h */ case    0:  		/* li R3, 0 */
		/* 8226C958h case    0:*/		cpu::op::li<0>(regs,&regs.R3,0x0);
		/* 8226C958h case    0:*/		return 0x8226C95C;
		  /* 8226C95Ch */ case    1:  		/* addi R1, R1, 144 */
		/* 8226C95Ch case    1:*/		cpu::op::addi<0>(regs,&regs.R1,regs.R1,0x90);
		/* 8226C95Ch case    1:*/		return 0x8226C960;
		  /* 8226C960h */ case    2:  		/* lwz R12, <#[R1 - 8]> */
		/* 8226C960h case    2:*/		cpu::mem::load32z( regs, &regs.R12, (uint32)(regs.R1 + 0xFFFFFFF8) );
		/* 8226C960h case    2:*/		return 0x8226C964;
		  /* 8226C964h */ case    3:  		/* mtspr LR, R12 */
		/* 8226C964h case    3:*/		regs.LR = regs.R12;
		/* 8226C964h case    3:*/		return 0x8226C968;
		  /* 8226C968h */ case    4:  		/* ld R30, <#[R1 - 24]> */
		/* 8226C968h case    4:*/		cpu::mem::load64( regs, &regs.R30, (uint32)(regs.R1 + 0xFFFFFFE8) );
		/* 8226C968h case    4:*/		return 0x8226C96C;
		  /* 8226C96Ch */ case    5:  		/* ld R31, <#[R1 - 16]> */
		/* 8226C96Ch case    5:*/		cpu::mem::load64( regs, &regs.R31, (uint32)(regs.R1 + 0xFFFFFFF0) );
		/* 8226C96Ch case    5:*/		return 0x8226C970;
		  /* 8226C970h */ case    6:  		/* bclr 20, CR0_LT */
		/* 8226C970h case    6:*/		if ( 1 ) { return (uint32)regs.LR; }
		/* 8226C970h case    6:*/		return 0x8226C974;
	}
	return 0x8226C974;
} // Block from 8226C958h-8226C974h (7 instructions)

//////////////////////////////////////////////////////
// Block at 8226C974h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C974( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C974) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C974);
		  /* 8226C974h */ case    0:  		/* nop */
		/* 8226C974h case    0:*/		cpu::op::nop();
		/* 8226C974h case    0:*/		return 0x8226C978;
		  /* 8226C978h */ case    1:  		/* mr R11, R6 */
		/* 8226C978h case    1:*/		regs.R11 = regs.R6;
		/* 8226C978h case    1:*/		return 0x8226C97C;
		  /* 8226C97Ch */ case    2:  		/* mr R6, R5 */
		/* 8226C97Ch case    2:*/		regs.R6 = regs.R5;
		/* 8226C97Ch case    2:*/		return 0x8226C980;
		  /* 8226C980h */ case    3:  		/* mr R5, R4 */
		/* 8226C980h case    3:*/		regs.R5 = regs.R4;
		/* 8226C980h case    3:*/		return 0x8226C984;
		  /* 8226C984h */ case    4:  		/* mr R4, R3 */
		/* 8226C984h case    4:*/		regs.R4 = regs.R3;
		/* 8226C984h case    4:*/		return 0x8226C988;
		  /* 8226C988h */ case    5:  		/* mr R3, R11 */
		/* 8226C988h case    5:*/		regs.R3 = regs.R11;
		/* 8226C988h case    5:*/		return 0x8226C98C;
		  /* 8226C98Ch */ case    6:  		/* b -3292 */
		/* 8226C98Ch case    6:*/		return 0x8226BCB0;
		/* 8226C98Ch case    6:*/		return 0x8226C990;
		  /* 8226C990h */ case    7:  		/* mr R11, R6 */
		/* 8226C990h case    7:*/		regs.R11 = regs.R6;
		/* 8226C990h case    7:*/		return 0x8226C994;
		  /* 8226C994h */ case    8:  		/* mr R6, R5 */
		/* 8226C994h case    8:*/		regs.R6 = regs.R5;
		/* 8226C994h case    8:*/		return 0x8226C998;
		  /* 8226C998h */ case    9:  		/* mr R5, R4 */
		/* 8226C998h case    9:*/		regs.R5 = regs.R4;
		/* 8226C998h case    9:*/		return 0x8226C99C;
		  /* 8226C99Ch */ case   10:  		/* mr R4, R3 */
		/* 8226C99Ch case   10:*/		regs.R4 = regs.R3;
		/* 8226C99Ch case   10:*/		return 0x8226C9A0;
		  /* 8226C9A0h */ case   11:  		/* mr R3, R11 */
		/* 8226C9A0h case   11:*/		regs.R3 = regs.R11;
		/* 8226C9A0h case   11:*/		return 0x8226C9A4;
		  /* 8226C9A4h */ case   12:  		/* b -220 */
		/* 8226C9A4h case   12:*/		return 0x8226C8C8;
		/* 8226C9A4h case   12:*/		return 0x8226C9A8;
	}
	return 0x8226C9A8;
} // Block from 8226C974h-8226C9A8h (13 instructions)

//////////////////////////////////////////////////////
// Block at 8226C9A8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C9A8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C9A8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C9A8);
		  /* 8226C9A8h */ case    0:  		/* mfspr R12, LR */
		/* 8226C9A8h case    0:*/		regs.R12 = regs.LR;
		/* 8226C9A8h case    0:*/		return 0x8226C9AC;
		  /* 8226C9ACh */ case    1:  		/* stw R12, <#[R1 - 8]> */
		/* 8226C9ACh case    1:*/		cpu::mem::store32( regs, regs.R12, (uint32)(regs.R1 + 0xFFFFFFF8) );
		/* 8226C9ACh case    1:*/		return 0x8226C9B0;
		  /* 8226C9B0h */ case    2:  		/* stwu R1, <#[R1 - 96]> */
		/* 8226C9B0h case    2:*/		cpu::mem::store32( regs, regs.R1, (uint32)(regs.R1 + 0xFFFFFFA0) );
		regs.R1 = (uint32)(regs.R1 + 0xFFFFFFA0);
		/* 8226C9B0h case    2:*/		return 0x8226C9B4;
		  /* 8226C9B4h */ case    3:  		/* mr R11, R4 */
		/* 8226C9B4h case    3:*/		regs.R11 = regs.R4;
		/* 8226C9B4h case    3:*/		return 0x8226C9B8;
		  /* 8226C9B8h */ case    4:  		/* stw R3, <#[R1 + 84]> */
		/* 8226C9B8h case    4:*/		cpu::mem::store32( regs, regs.R3, (uint32)(regs.R1 + 0x00000054) );
		/* 8226C9B8h case    4:*/		return 0x8226C9BC;
		  /* 8226C9BCh */ case    5:  		/* lis R10, -32217 */
		/* 8226C9BCh case    5:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8227);
		/* 8226C9BCh case    5:*/		return 0x8226C9C0;
		  /* 8226C9C0h */ case    6:  		/* mr R4, R5 */
		/* 8226C9C0h case    6:*/		regs.R4 = regs.R5;
		/* 8226C9C0h case    6:*/		return 0x8226C9C4;
		  /* 8226C9C4h */ case    7:  		/* li R9, 1 */
		/* 8226C9C4h case    7:*/		cpu::op::li<0>(regs,&regs.R9,0x1);
		/* 8226C9C4h case    7:*/		return 0x8226C9C8;
		  /* 8226C9C8h */ case    8:  		/* addi R10, R10, -13936 */
		/* 8226C9C8h case    8:*/		cpu::op::addi<0>(regs,&regs.R10,regs.R10,0xFFFFC990);
		/* 8226C9C8h case    8:*/		return 0x8226C9CC;
		  /* 8226C9CCh */ case    9:  		/* li R8, 0 */
		/* 8226C9CCh case    9:*/		cpu::op::li<0>(regs,&regs.R8,0x0);
		/* 8226C9CCh case    9:*/		return 0x8226C9D0;
		  /* 8226C9D0h */ case   10:  		/* li R7, 0 */
		/* 8226C9D0h case   10:*/		cpu::op::li<0>(regs,&regs.R7,0x0);
		/* 8226C9D0h case   10:*/		return 0x8226C9D4;
		  /* 8226C9D4h */ case   11:  		/* li R6, 0 */
		/* 8226C9D4h case   11:*/		cpu::op::li<0>(regs,&regs.R6,0x0);
		/* 8226C9D4h case   11:*/		return 0x8226C9D8;
		  /* 8226C9D8h */ case   12:  		/* li R5, 0 */
		/* 8226C9D8h case   12:*/		cpu::op::li<0>(regs,&regs.R5,0x0);
		/* 8226C9D8h case   12:*/		return 0x8226C9DC;
		  /* 8226C9DCh */ case   13:  		/* mr R3, R11 */
		/* 8226C9DCh case   13:*/		regs.R3 = regs.R11;
		/* 8226C9DCh case   13:*/		return 0x8226C9E0;
		  /* 8226C9E0h */ case   14:  		/* bl -2114920 */
		/* 8226C9E0h case   14:*/		regs.LR = 0x8226C9E4; return 0x82068478;
		/* 8226C9E0h case   14:*/		return 0x8226C9E4;
		  /* 8226C9E4h */ case   15:  		/* addi R1, R1, 96 */
		/* 8226C9E4h case   15:*/		cpu::op::addi<0>(regs,&regs.R1,regs.R1,0x60);
		/* 8226C9E4h case   15:*/		return 0x8226C9E8;
		  /* 8226C9E8h */ case   16:  		/* lwz R12, <#[R1 - 8]> */
		/* 8226C9E8h case   16:*/		cpu::mem::load32z( regs, &regs.R12, (uint32)(regs.R1 + 0xFFFFFFF8) );
		/* 8226C9E8h case   16:*/		return 0x8226C9EC;
		  /* 8226C9ECh */ case   17:  		/* mtspr LR, R12 */
		/* 8226C9ECh case   17:*/		regs.LR = regs.R12;
		/* 8226C9ECh case   17:*/		return 0x8226C9F0;
		  /* 8226C9F0h */ case   18:  		/* bclr 20, CR0_LT */
		/* 8226C9F0h case   18:*/		if ( 1 ) { return (uint32)regs.LR; }
		/* 8226C9F0h case   18:*/		return 0x8226C9F4;
	}
	return 0x8226C9F4;
} // Block from 8226C9A8h-8226C9F4h (19 instructions)

//////////////////////////////////////////////////////
// Block at 8226C9F4h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C9F4( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C9F4) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C9F4);
		  /* 8226C9F4h */ case    0:  		/* nop */
		/* 8226C9F4h case    0:*/		cpu::op::nop();
		/* 8226C9F4h case    0:*/		return 0x8226C9F8;
	}
	return 0x8226C9F8;
} // Block from 8226C9F4h-8226C9F8h (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226C9F8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226C9F8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226C9F8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226C9F8);
		  /* 8226C9F8h */ case    0:  		/* mfspr R12, LR */
		/* 8226C9F8h case    0:*/		regs.R12 = regs.LR;
		/* 8226C9F8h case    0:*/		return 0x8226C9FC;
		  /* 8226C9FCh */ case    1:  		/* bl -1947560 */
		/* 8226C9FCh case    1:*/		regs.LR = 0x8226CA00; return 0x82091254;
		/* 8226C9FCh case    1:*/		return 0x8226CA00;
		  /* 8226CA00h */ case    2:  		/* stwu R1, <#[R1 - 176]> */
		/* 8226CA00h case    2:*/		cpu::mem::store32( regs, regs.R1, (uint32)(regs.R1 + 0xFFFFFF50) );
		regs.R1 = (uint32)(regs.R1 + 0xFFFFFF50);
		/* 8226CA00h case    2:*/		return 0x8226CA04;
		  /* 8226CA04h */ case    3:  		/* nor R9, R8, R8 */
		/* 8226CA04h case    3:*/		cpu::op::nor<0>(regs,&regs.R9,regs.R8,regs.R8);
		/* 8226CA04h case    3:*/		return 0x8226CA08;
		  /* 8226CA08h */ case    4:  		/* stw R4, <#[R3]> */
		/* 8226CA08h case    4:*/		cpu::mem::store32( regs, regs.R4, (uint32)(regs.R3 + 0x00000000) );
		/* 8226CA08h case    4:*/		return 0x8226CA0C;
		  /* 8226CA0Ch */ case    5:  		/* li R10, 1 */
		/* 8226CA0Ch case    5:*/		cpu::op::li<0>(regs,&regs.R10,0x1);
		/* 8226CA0Ch case    5:*/		return 0x8226CA10;
		  /* 8226CA10h */ case    6:  		/* stw R5, <#[R3 + 4]> */
		/* 8226CA10h case    6:*/		cpu::mem::store32( regs, regs.R5, (uint32)(regs.R3 + 0x00000004) );
		/* 8226CA10h case    6:*/		return 0x8226CA14;
		  /* 8226CA14h */ case    7:  		/* li R11, 0 */
		/* 8226CA14h case    7:*/		cpu::op::li<0>(regs,&regs.R11,0x0);
		/* 8226CA14h case    7:*/		return 0x8226CA18;
		  /* 8226CA18h */ case    8:  		/* stw R6, <#[R3 + 16]> */
		/* 8226CA18h case    8:*/		cpu::mem::store32( regs, regs.R6, (uint32)(regs.R3 + 0x00000010) );
		/* 8226CA18h case    8:*/		return 0x8226CA1C;
		  /* 8226CA1Ch */ case    9:  		/* rlwinm R9, R9, 0, 31, 31 */
		/* 8226CA1Ch case    9:*/		cpu::op::rlwinm<0,0,31,31>(regs,&regs.R9,regs.R9);
		/* 8226CA1Ch case    9:*/		return 0x8226CA20;
		  /* 8226CA20h */ case   10:  		/* stw R8, <#[R3 + 8]> */
		/* 8226CA20h case   10:*/		cpu::mem::store32( regs, regs.R8, (uint32)(regs.R3 + 0x00000008) );
		/* 8226CA20h case   10:*/		return 0x8226CA24;
		  /* 8226CA24h */ case   11:  		/* stb R10, <#[R3 + 12]> */
		/* 8226CA24h case   11:*/		cpu::mem::store8( regs, regs.R10, (uint32)(regs.R3 + 0x0000000C) );
		/* 8226CA24h case   11:*/		return 0x8226CA28;
		  /* 8226CA28h */ case   12:  		/* mr R5, R7 */
		/* 8226CA28h case   12:*/		regs.R5 = regs.R7;
		/* 8226CA28h case   12:*/		return 0x8226CA2C;
		  /* 8226CA2Ch */ case   13:  		/* stb R9, <#[R3 + 13]> */
		/* 8226CA2Ch case   13:*/		cpu::mem::store8( regs, regs.R9, (uint32)(regs.R3 + 0x0000000D) );
		/* 8226CA2Ch case   13:*/		return 0x8226CA30;
		  /* 8226CA30h */ case   14:  		/* mr R4, R6 */
		/* 8226CA30h case   14:*/		regs.R4 = regs.R6;
		/* 8226CA30h case   14:*/		return 0x8226CA34;
		  /* 8226CA34h */ case   15:  		/* stb R10, <#[R3 + 8232]> */
		/* 8226CA34h case   15:*/		cpu::mem::store8( regs, regs.R10, (uint32)(regs.R3 + 0x00002028) );
		/* 8226CA34h case   15:*/		return 0x8226CA38;
		  /* 8226CA38h */ case   16:  		/* mr R31, R3 */
		/* 8226CA38h case   16:*/		regs.R31 = regs.R3;
		/* 8226CA38h case   16:*/		return 0x8226CA3C;
		  /* 8226CA3Ch */ case   17:  		/* stw R11, <#[R3 + 8240]> */
		/* 8226CA3Ch case   17:*/		cpu::mem::store32( regs, regs.R11, (uint32)(regs.R3 + 0x00002030) );
		/* 8226CA3Ch case   17:*/		return 0x8226CA40;
		  /* 8226CA40h */ case   18:  		/* mr R30, R6 */
		/* 8226CA40h case   18:*/		regs.R30 = regs.R6;
		/* 8226CA40h case   18:*/		return 0x8226CA44;
		  /* 8226CA44h */ case   19:  		/* stw R11, <#[R3 + 8236]> */
		/* 8226CA44h case   19:*/		cpu::mem::store32( regs, regs.R11, (uint32)(regs.R3 + 0x0000202C) );
		/* 8226CA44h case   19:*/		return 0x8226CA48;
		  /* 8226CA48h */ case   20:  		/* mr R29, R7 */
		/* 8226CA48h case   20:*/		regs.R29 = regs.R7;
		/* 8226CA48h case   20:*/		return 0x8226CA4C;
		  /* 8226CA4Ch */ case   21:  		/* bl -164 */
		/* 8226CA4Ch case   21:*/		regs.LR = 0x8226CA50; return 0x8226C9A8;
		/* 8226CA4Ch case   21:*/		return 0x8226CA50;
		  /* 8226CA50h */ case   22:  		/* lis R11, -32217 */
		/* 8226CA50h case   22:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8227);
		/* 8226CA50h case   22:*/		return 0x8226CA54;
		  /* 8226CA54h */ case   23:  		/* li R9, 7 */
		/* 8226CA54h case   23:*/		cpu::op::li<0>(regs,&regs.R9,0x7);
		/* 8226CA54h case   23:*/		return 0x8226CA58;
		  /* 8226CA58h */ case   24:  		/* stw R31, <#[R1 + 84]> */
		/* 8226CA58h case   24:*/		cpu::mem::store32( regs, regs.R31, (uint32)(regs.R1 + 0x00000054) );
		/* 8226CA58h case   24:*/		return 0x8226CA5C;
		  /* 8226CA5Ch */ case   25:  		/* addi R10, R11, -13960 */
		/* 8226CA5Ch case   25:*/		cpu::op::addi<0>(regs,&regs.R10,regs.R11,0xFFFFC978);
		/* 8226CA5Ch case   25:*/		return 0x8226CA60;
		  /* 8226CA60h */ case   26:  		/* li R8, 0 */
		/* 8226CA60h case   26:*/		cpu::op::li<0>(regs,&regs.R8,0x0);
		/* 8226CA60h case   26:*/		return 0x8226CA64;
		  /* 8226CA64h */ case   27:  		/* li R7, 0 */
		/* 8226CA64h case   27:*/		cpu::op::li<0>(regs,&regs.R7,0x0);
		/* 8226CA64h case   27:*/		return 0x8226CA68;
		  /* 8226CA68h */ case   28:  		/* li R6, 0 */
		/* 8226CA68h case   28:*/		cpu::op::li<0>(regs,&regs.R6,0x0);
		/* 8226CA68h case   28:*/		return 0x8226CA6C;
		  /* 8226CA6Ch */ case   29:  		/* li R5, 0 */
		/* 8226CA6Ch case   29:*/		cpu::op::li<0>(regs,&regs.R5,0x0);
		/* 8226CA6Ch case   29:*/		return 0x8226CA70;
		  /* 8226CA70h */ case   30:  		/* mr R4, R29 */
		/* 8226CA70h case   30:*/		regs.R4 = regs.R29;
		/* 8226CA70h case   30:*/		return 0x8226CA74;
		  /* 8226CA74h */ case   31:  		/* mr R3, R30 */
		/* 8226CA74h case   31:*/		regs.R3 = regs.R30;
		/* 8226CA74h case   31:*/		return 0x8226CA78;
		  /* 8226CA78h */ case   32:  		/* bl -2115072 */
		/* 8226CA78h case   32:*/		regs.LR = 0x8226CA7C; return 0x82068478;
		/* 8226CA78h case   32:*/		return 0x8226CA7C;
		  /* 8226CA7Ch */ case   33:  		/* or. R27, R3, R3 */
		/* 8226CA7Ch case   33:*/		cpu::op::or<1>(regs,&regs.R27,regs.R3,regs.R3);
		/* 8226CA7Ch case   33:*/		return 0x8226CA80;
		  /* 8226CA80h */ case   34:  		/* bc 12, CR0_LT, 96 */
		/* 8226CA80h case   34:*/		if ( regs.CR[0].lt ) { return 0x8226CAE0;  }
		/* 8226CA80h case   34:*/		return 0x8226CA84;
		  /* 8226CA84h */ case   35:  		/* lwz R11, <#[R31 + 8]> */
		/* 8226CA84h case   35:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00000008) );
		/* 8226CA84h case   35:*/		return 0x8226CA88;
		  /* 8226CA88h */ case   36:  		/* rlwinm. R11, R11, 0, 23, 23 */
		/* 8226CA88h case   36:*/		cpu::op::rlwinm<1,0,23,23>(regs,&regs.R11,regs.R11);
		/* 8226CA88h case   36:*/		return 0x8226CA8C;
		  /* 8226CA8Ch */ case   37:  		/* bc 4, CR0_EQ, 84 */
		/* 8226CA8Ch case   37:*/		if ( !regs.CR[0].eq ) { return 0x8226CAE0;  }
		/* 8226CA8Ch case   37:*/		return 0x8226CA90;
		  /* 8226CA90h */ case   38:  		/* lwz R11, <#[R31 + 8240]> */
		/* 8226CA90h case   38:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R31 + 0x00002030) );
		/* 8226CA90h case   38:*/		return 0x8226CA94;
		  /* 8226CA94h */ case   39:  		/* addi R11, R11, 1 */
		/* 8226CA94h case   39:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R11,0x1);
		/* 8226CA94h case   39:*/		return 0x8226CA98;
		  /* 8226CA98h */ case   40:  		/* mulli R28, R11, 12 */
		/* 8226CA98h case   40:*/		cpu::op::mulli<0>(regs,&regs.R28,regs.R11,0xC);
		/* 8226CA98h case   40:*/		return 0x8226CA9C;
		  /* 8226CA9Ch */ case   41:  		/* addi R11, R28, 12 */
		/* 8226CA9Ch case   41:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R28,0xC);
		/* 8226CA9Ch case   41:*/		return 0x8226CAA0;
		  /* 8226CAA0h */ case   42:  		/* cmplw CR6, R11, R29 */
		/* 8226CAA0h case   42:*/		cpu::op::cmplw<6>(regs,regs.R11,regs.R29);
		/* 8226CAA0h case   42:*/		return 0x8226CAA4;
		  /* 8226CAA4h */ case   43:  		/* bc 12, CR6_GT, 60 */
		/* 8226CAA4h case   43:*/		if ( regs.CR[6].gt ) { return 0x8226CAE0;  }
		/* 8226CAA4h case   43:*/		return 0x8226CAA8;
		  /* 8226CAA8h */ case   44:  		/* mr R3, R31 */
		/* 8226CAA8h case   44:*/		regs.R3 = regs.R31;
		/* 8226CAA8h case   44:*/		return 0x8226CAAC;
		  /* 8226CAACh */ case   45:  		/* bl -7748 */
		/* 8226CAACh case   45:*/		regs.LR = 0x8226CAB0; return 0x8226AC68;
		/* 8226CAACh case   45:*/		return 0x8226CAB0;
		  /* 8226CAB0h */ case   46:  		/* mr R3, R31 */
		/* 8226CAB0h case   46:*/		regs.R3 = regs.R31;
		/* 8226CAB0h case   46:*/		return 0x8226CAB4;
		  /* 8226CAB4h */ case   47:  		/* add R29, R28, R30 */
		/* 8226CAB4h case   47:*/		cpu::op::add<0>(regs,&regs.R29,regs.R28,regs.R30);
		/* 8226CAB4h case   47:*/		return 0x8226CAB8;
		  /* 8226CAB8h */ case   48:  		/* bl -11856 */
		/* 8226CAB8h case   48:*/		regs.LR = 0x8226CABC; return 0x82269C68;
		/* 8226CAB8h case   48:*/		return 0x8226CABC;
		  /* 8226CABCh */ case   49:  		/* lis R11, -32251 */
		/* 8226CABCh case   49:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8205);
		/* 8226CABCh case   49:*/		return 0x8226CAC0;
		  /* 8226CAC0h */ case   50:  		/* mr R3, R31 */
		/* 8226CAC0h case   50:*/		regs.R3 = regs.R31;
		/* 8226CAC0h case   50:*/		return 0x8226CAC4;
		  /* 8226CAC4h */ case   51:  		/* lwz R7, <#[R29 + 8]> */
		/* 8226CAC4h case   51:*/		cpu::mem::load32z( regs, &regs.R7, (uint32)(regs.R29 + 0x00000008) );
		/* 8226CAC4h case   51:*/		return 0x8226CAC8;
		  /* 8226CAC8h */ case   52:  		/* addi R4, R11, -15776 */
		/* 8226CAC8h case   52:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFFC260);
		/* 8226CAC8h case   52:*/		return 0x8226CACC;
		  /* 8226CACCh */ case   53:  		/* lwz R6, <#[R29 + 4]> */
		/* 8226CACCh case   53:*/		cpu::mem::load32z( regs, &regs.R6, (uint32)(regs.R29 + 0x00000004) );
		/* 8226CACCh case   53:*/		return 0x8226CAD0;
		  /* 8226CAD0h */ case   54:  		/* lwzx R5, <#[R28 + R30]> */
		/* 8226CAD0h case   54:*/		cpu::mem::load32z( regs, &regs.R5, (uint32)(regs.R28 + regs.R30 + 0x00000000) );
		/* 8226CAD0h case   54:*/		return 0x8226CAD4;
		  /* 8226CAD4h */ case   55:  		/* bl -12532 */
		/* 8226CAD4h case   55:*/		regs.LR = 0x8226CAD8; return 0x822699E0;
		/* 8226CAD4h case   55:*/		return 0x8226CAD8;
		  /* 8226CAD8h */ case   56:  		/* mr R3, R31 */
		/* 8226CAD8h case   56:*/		regs.R3 = regs.R31;
		/* 8226CAD8h case   56:*/		return 0x8226CADC;
		  /* 8226CADCh */ case   57:  		/* bl -7596 */
		/* 8226CADCh case   57:*/		regs.LR = 0x8226CAE0; return 0x8226AD30;
		/* 8226CADCh case   57:*/		return 0x8226CAE0;
	}
	return 0x8226CAE0;
} // Block from 8226C9F8h-8226CAE0h (58 instructions)

//////////////////////////////////////////////////////
// Block at 8226CAE0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CAE0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CAE0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CAE0);
		  /* 8226CAE0h */ case    0:  		/* mr R3, R27 */
		/* 8226CAE0h case    0:*/		regs.R3 = regs.R27;
		/* 8226CAE0h case    0:*/		return 0x8226CAE4;
		  /* 8226CAE4h */ case    1:  		/* addi R1, R1, 176 */
		/* 8226CAE4h case    1:*/		cpu::op::addi<0>(regs,&regs.R1,regs.R1,0xB0);
		/* 8226CAE4h case    1:*/		return 0x8226CAE8;
		  /* 8226CAE8h */ case    2:  		/* b -1947716 */
		/* 8226CAE8h case    2:*/		return 0x820912A4;
		/* 8226CAE8h case    2:*/		return 0x8226CAEC;
		  /* 8226CAECh */ case    3:  		/* nop */
		/* 8226CAECh case    3:*/		cpu::op::nop();
		/* 8226CAECh case    3:*/		return 0x8226CAF0;
	}
	return 0x8226CAF0;
} // Block from 8226CAE0h-8226CAF0h (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226CAF0h
// Function ''
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CAF0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CAF0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CAF0);
		  /* 8226CAF0h */ case    0:  		/* mfspr R12, LR */
		/* 8226CAF0h case    0:*/		regs.R12 = regs.LR;
		/* 8226CAF0h case    0:*/		return 0x8226CAF4;
		  /* 8226CAF4h */ case    1:  		/* bl -1947816 */
		/* 8226CAF4h case    1:*/		regs.LR = 0x8226CAF8; return 0x8209124C;
		/* 8226CAF4h case    1:*/		return 0x8226CAF8;
		  /* 8226CAF8h */ case    2:  		/* ld R12, <#[R1 - 4096]> */
		/* 8226CAF8h case    2:*/		cpu::mem::load64( regs, &regs.R12, (uint32)(regs.R1 + 0xFFFFF000) );
		/* 8226CAF8h case    2:*/		return 0x8226CAFC;
		  /* 8226CAFCh */ case    3:  		/* ld R12, <#[R1 - 8192]> */
		/* 8226CAFCh case    3:*/		cpu::mem::load64( regs, &regs.R12, (uint32)(regs.R1 + 0xFFFFE000) );
		/* 8226CAFCh case    3:*/		return 0x8226CB00;
		  /* 8226CB00h */ case    4:  		/* stwu R1, <#[R1 - 8400]> */
		/* 8226CB00h case    4:*/		cpu::mem::store32( regs, regs.R1, (uint32)(regs.R1 + 0xFFFFDF30) );
		regs.R1 = (uint32)(regs.R1 + 0xFFFFDF30);
		/* 8226CB00h case    4:*/		return 0x8226CB04;
		  /* 8226CB04h */ case    5:  		/* mr R25, R10 */
		/* 8226CB04h case    5:*/		regs.R25 = regs.R10;
		/* 8226CB04h case    5:*/		return 0x8226CB08;
		  /* 8226CB08h */ case    6:  		/* lwz R10, <#[R1 + 8484]> */
		/* 8226CB08h case    6:*/		cpu::mem::load32z( regs, &regs.R10, (uint32)(regs.R1 + 0x00002124) );
		/* 8226CB08h case    6:*/		return 0x8226CB0C;
		  /* 8226CB0Ch */ case    7:  		/* mr R30, R3 */
		/* 8226CB0Ch case    7:*/		regs.R30 = regs.R3;
		/* 8226CB0Ch case    7:*/		return 0x8226CB10;
		  /* 8226CB10h */ case    8:  		/* rlwinm R11, R10, 0, 30, 31 */
		/* 8226CB10h case    8:*/		cpu::op::rlwinm<0,0,30,31>(regs,&regs.R11,regs.R10);
		/* 8226CB10h case    8:*/		return 0x8226CB14;
		  /* 8226CB14h */ case    9:  		/* mr R29, R4 */
		/* 8226CB14h case    9:*/		regs.R29 = regs.R4;
		/* 8226CB14h case    9:*/		return 0x8226CB18;
		  /* 8226CB18h */ case   10:  		/* mr R28, R5 */
		/* 8226CB18h case   10:*/		regs.R28 = regs.R5;
		/* 8226CB18h case   10:*/		return 0x8226CB1C;
		  /* 8226CB1Ch */ case   11:  		/* mr R27, R6 */
		/* 8226CB1Ch case   11:*/		regs.R27 = regs.R6;
		/* 8226CB1Ch case   11:*/		return 0x8226CB20;
		  /* 8226CB20h */ case   12:  		/* mr R26, R9 */
		/* 8226CB20h case   12:*/		regs.R26 = regs.R9;
		/* 8226CB20h case   12:*/		return 0x8226CB24;
		  /* 8226CB24h */ case   13:  		/* li R3, 0 */
		/* 8226CB24h case   13:*/		cpu::op::li<0>(regs,&regs.R3,0x0);
		/* 8226CB24h case   13:*/		return 0x8226CB28;
		  /* 8226CB28h */ case   14:  		/* li R31, 0 */
		/* 8226CB28h case   14:*/		cpu::op::li<0>(regs,&regs.R31,0x0);
		/* 8226CB28h case   14:*/		return 0x8226CB2C;
		  /* 8226CB2Ch */ case   15:  		/* cmplwi CR6, R11, 1 */
		/* 8226CB2Ch case   15:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x00000001);
		/* 8226CB2Ch case   15:*/		return 0x8226CB30;
		  /* 8226CB30h */ case   16:  		/* bc 12, CR6_EQ, 36 */
		/* 8226CB30h case   16:*/		if ( regs.CR[6].eq ) { return 0x8226CB54;  }
		/* 8226CB30h case   16:*/		return 0x8226CB34;
		  /* 8226CB34h */ case   17:  		/* cmplwi CR6, R11, 2 */
		/* 8226CB34h case   17:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x00000002);
		/* 8226CB34h case   17:*/		return 0x8226CB38;
		  /* 8226CB38h */ case   18:  		/* bc 4, CR6_EQ, 12 */
		/* 8226CB38h case   18:*/		if ( !regs.CR[6].eq ) { return 0x8226CB44;  }
		/* 8226CB38h case   18:*/		return 0x8226CB3C;
		  /* 8226CB3Ch */ case   19:  		/* li R31, 8 */
		/* 8226CB3Ch case   19:*/		cpu::op::li<0>(regs,&regs.R31,0x8);
		/* 8226CB3Ch case   19:*/		return 0x8226CB40;
		  /* 8226CB40h */ case   20:  		/* b 20 */
		/* 8226CB40h case   20:*/		return 0x8226CB54;
		/* 8226CB40h case   20:*/		return 0x8226CB44;
	}
	return 0x8226CB44;
} // Block from 8226CAF0h-8226CB44h (21 instructions)

//////////////////////////////////////////////////////
// Block at 8226CB44h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CB44( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CB44) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CB44);
		  /* 8226CB44h */ case    0:  		/* cmplwi CR6, R11, 3 */
		/* 8226CB44h case    0:*/		cpu::op::cmplwi<6>(regs,regs.R11,0x00000003);
		/* 8226CB44h case    0:*/		return 0x8226CB48;
		  /* 8226CB48h */ case    1:  		/* bc 12, CR6_EQ, 12 */
		/* 8226CB48h case    1:*/		if ( regs.CR[6].eq ) { return 0x8226CB54;  }
		/* 8226CB48h case    1:*/		return 0x8226CB4C;
		  /* 8226CB4Ch */ case    2:  		/* lis R3, -32761 */
		/* 8226CB4Ch case    2:*/		cpu::op::lis<0>(regs,&regs.R3,0xFFFF8007);
		/* 8226CB4Ch case    2:*/		return 0x8226CB50;
		  /* 8226CB50h */ case    3:  		/* ori R3, R3, 87 */
		/* 8226CB50h case    3:*/		cpu::op::ori<0>(regs,&regs.R3,regs.R3,0x57);
		/* 8226CB50h case    3:*/		return 0x8226CB54;
	}
	return 0x8226CB54;
} // Block from 8226CB44h-8226CB54h (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226CB54h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CB54( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CB54) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CB54);
		  /* 8226CB54h */ case    0:  		/* rlwinm. R11, R10, 0, 28, 28 */
		/* 8226CB54h case    0:*/		cpu::op::rlwinm<1,0,28,28>(regs,&regs.R11,regs.R10);
		/* 8226CB54h case    0:*/		return 0x8226CB58;
		  /* 8226CB58h */ case    1:  		/* bc 12, CR0_EQ, 8 */
		/* 8226CB58h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226CB60;  }
		/* 8226CB58h case    1:*/		return 0x8226CB5C;
		  /* 8226CB5Ch */ case    2:  		/* ori R31, R31, 2 */
		/* 8226CB5Ch case    2:*/		cpu::op::ori<0>(regs,&regs.R31,regs.R31,0x2);
		/* 8226CB5Ch case    2:*/		return 0x8226CB60;
	}
	return 0x8226CB60;
} // Block from 8226CB54h-8226CB60h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226CB60h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CB60( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CB60) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CB60);
		  /* 8226CB60h */ case    0:  		/* rlwinm. R11, R10, 0, 29, 29 */
		/* 8226CB60h case    0:*/		cpu::op::rlwinm<1,0,29,29>(regs,&regs.R11,regs.R10);
		/* 8226CB60h case    0:*/		return 0x8226CB64;
		  /* 8226CB64h */ case    1:  		/* bc 12, CR0_EQ, 8 */
		/* 8226CB64h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226CB6C;  }
		/* 8226CB64h case    1:*/		return 0x8226CB68;
		  /* 8226CB68h */ case    2:  		/* ori R31, R31, 4 */
		/* 8226CB68h case    2:*/		cpu::op::ori<0>(regs,&regs.R31,regs.R31,0x4);
		/* 8226CB68h case    2:*/		return 0x8226CB6C;
	}
	return 0x8226CB6C;
} // Block from 8226CB60h-8226CB6Ch (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226CB6Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CB6C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CB6C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CB6C);
		  /* 8226CB6Ch */ case    0:  		/* rlwinm. R11, R10, 0, 25, 25 */
		/* 8226CB6Ch case    0:*/		cpu::op::rlwinm<1,0,25,25>(regs,&regs.R11,regs.R10);
		/* 8226CB6Ch case    0:*/		return 0x8226CB70;
		  /* 8226CB70h */ case    1:  		/* bc 12, CR0_EQ, 8 */
		/* 8226CB70h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226CB78;  }
		/* 8226CB70h case    1:*/		return 0x8226CB74;
		  /* 8226CB74h */ case    2:  		/* ori R31, R31, 32 */
		/* 8226CB74h case    2:*/		cpu::op::ori<0>(regs,&regs.R31,regs.R31,0x20);
		/* 8226CB74h case    2:*/		return 0x8226CB78;
	}
	return 0x8226CB78;
} // Block from 8226CB6Ch-8226CB78h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226CB78h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CB78( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CB78) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CB78);
		  /* 8226CB78h */ case    0:  		/* rlwinm. R11, R10, 0, 27, 27 */
		/* 8226CB78h case    0:*/		cpu::op::rlwinm<1,0,27,27>(regs,&regs.R11,regs.R10);
		/* 8226CB78h case    0:*/		return 0x8226CB7C;
		  /* 8226CB7Ch */ case    1:  		/* bc 12, CR0_EQ, 8 */
		/* 8226CB7Ch case    1:*/		if ( regs.CR[0].eq ) { return 0x8226CB84;  }
		/* 8226CB7Ch case    1:*/		return 0x8226CB80;
		  /* 8226CB80h */ case    2:  		/* ori R31, R31, 16 */
		/* 8226CB80h case    2:*/		cpu::op::ori<0>(regs,&regs.R31,regs.R31,0x10);
		/* 8226CB80h case    2:*/		return 0x8226CB84;
	}
	return 0x8226CB84;
} // Block from 8226CB78h-8226CB84h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226CB84h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CB84( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CB84) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CB84);
		  /* 8226CB84h */ case    0:  		/* rlwinm. R11, R10, 0, 24, 24 */
		/* 8226CB84h case    0:*/		cpu::op::rlwinm<1,0,24,24>(regs,&regs.R11,regs.R10);
		/* 8226CB84h case    0:*/		return 0x8226CB88;
		  /* 8226CB88h */ case    1:  		/* bc 12, CR0_EQ, 8 */
		/* 8226CB88h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226CB90;  }
		/* 8226CB88h case    1:*/		return 0x8226CB8C;
		  /* 8226CB8Ch */ case    2:  		/* ori R31, R31, 64 */
		/* 8226CB8Ch case    2:*/		cpu::op::ori<0>(regs,&regs.R31,regs.R31,0x40);
		/* 8226CB8Ch case    2:*/		return 0x8226CB90;
	}
	return 0x8226CB90;
} // Block from 8226CB84h-8226CB90h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226CB90h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CB90( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CB90) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CB90);
		  /* 8226CB90h */ case    0:  		/* rlwinm. R11, R10, 0, 22, 22 */
		/* 8226CB90h case    0:*/		cpu::op::rlwinm<1,0,22,22>(regs,&regs.R11,regs.R10);
		/* 8226CB90h case    0:*/		return 0x8226CB94;
		  /* 8226CB94h */ case    1:  		/* bc 12, CR0_EQ, 8 */
		/* 8226CB94h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226CB9C;  }
		/* 8226CB94h case    1:*/		return 0x8226CB98;
		  /* 8226CB98h */ case    2:  		/* ori R31, R31, 128 */
		/* 8226CB98h case    2:*/		cpu::op::ori<0>(regs,&regs.R31,regs.R31,0x80);
		/* 8226CB98h case    2:*/		return 0x8226CB9C;
	}
	return 0x8226CB9C;
} // Block from 8226CB90h-8226CB9Ch (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226CB9Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CB9C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CB9C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CB9C);
		  /* 8226CB9Ch */ case    0:  		/* rlwinm. R11, R10, 0, 21, 21 */
		/* 8226CB9Ch case    0:*/		cpu::op::rlwinm<1,0,21,21>(regs,&regs.R11,regs.R10);
		/* 8226CB9Ch case    0:*/		return 0x8226CBA0;
		  /* 8226CBA0h */ case    1:  		/* bc 12, CR0_EQ, 8 */
		/* 8226CBA0h case    1:*/		if ( regs.CR[0].eq ) { return 0x8226CBA8;  }
		/* 8226CBA0h case    1:*/		return 0x8226CBA4;
		  /* 8226CBA4h */ case    2:  		/* ori R31, R31, 256 */
		/* 8226CBA4h case    2:*/		cpu::op::ori<0>(regs,&regs.R31,regs.R31,0x100);
		/* 8226CBA4h case    2:*/		return 0x8226CBA8;
	}
	return 0x8226CBA8;
} // Block from 8226CB9Ch-8226CBA8h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226CBA8h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CBA8( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CBA8) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CBA8);
		  /* 8226CBA8h */ case    0:  		/* cmpwi CR6, R3, 0 */
		/* 8226CBA8h case    0:*/		cpu::op::cmpwi<6>(regs,regs.R3,0x00000000);
		/* 8226CBA8h case    0:*/		return 0x8226CBAC;
		  /* 8226CBACh */ case    1:  		/* bc 12, CR6_LT, 80 */
		/* 8226CBACh case    1:*/		if ( regs.CR[6].lt ) { return 0x8226CBFC;  }
		/* 8226CBACh case    1:*/		return 0x8226CBB0;
		  /* 8226CBB0h */ case    2:  		/* li R5, 8192 */
		/* 8226CBB0h case    2:*/		cpu::op::li<0>(regs,&regs.R5,0x2000);
		/* 8226CBB0h case    2:*/		return 0x8226CBB4;
		  /* 8226CBB4h */ case    3:  		/* li R4, 0 */
		/* 8226CBB4h case    3:*/		cpu::op::li<0>(regs,&regs.R4,0x0);
		/* 8226CBB4h case    3:*/		return 0x8226CBB8;
		  /* 8226CBB8h */ case    4:  		/* addi R3, R1, 104 */
		/* 8226CBB8h case    4:*/		cpu::op::addi<0>(regs,&regs.R3,regs.R1,0x68);
		/* 8226CBB8h case    4:*/		return 0x8226CBBC;
		  /* 8226CBBCh */ case    5:  		/* bl -1947772 */
		/* 8226CBBCh case    5:*/		regs.LR = 0x8226CBC0; return 0x82091340;
		/* 8226CBBCh case    5:*/		return 0x8226CBC0;
		  /* 8226CBC0h */ case    6:  		/* lwz R11, <#[R1 + 8492]> */
		/* 8226CBC0h case    6:*/		cpu::mem::load32z( regs, &regs.R11, (uint32)(regs.R1 + 0x0000212C) );
		/* 8226CBC0h case    6:*/		return 0x8226CBC4;
		  /* 8226CBC4h */ case    7:  		/* lwz R10, <#[R1 + 8500]> */
		/* 8226CBC4h case    7:*/		cpu::mem::load32z( regs, &regs.R10, (uint32)(regs.R1 + 0x00002134) );
		/* 8226CBC4h case    7:*/		return 0x8226CBC8;
		  /* 8226CBC8h */ case    8:  		/* li R9, 0 */
		/* 8226CBC8h case    8:*/		cpu::op::li<0>(regs,&regs.R9,0x0);
		/* 8226CBC8h case    8:*/		return 0x8226CBCC;
		  /* 8226CBCCh */ case    9:  		/* stw R26, <#[R1 + 8304]> */
		/* 8226CBCCh case    9:*/		cpu::mem::store32( regs, regs.R26, (uint32)(regs.R1 + 0x00002070) );
		/* 8226CBCCh case    9:*/		return 0x8226CBD0;
		  /* 8226CBD0h */ case   10:  		/* mr R8, R31 */
		/* 8226CBD0h case   10:*/		regs.R8 = regs.R31;
		/* 8226CBD0h case   10:*/		return 0x8226CBD4;
		  /* 8226CBD4h */ case   11:  		/* stw R25, <#[R1 + 8308]> */
		/* 8226CBD4h case   11:*/		cpu::mem::store32( regs, regs.R25, (uint32)(regs.R1 + 0x00002074) );
		/* 8226CBD4h case   11:*/		return 0x8226CBD8;
		  /* 8226CBD8h */ case   12:  		/* mr R7, R27 */
		/* 8226CBD8h case   12:*/		regs.R7 = regs.R27;
		/* 8226CBD8h case   12:*/		return 0x8226CBDC;
		  /* 8226CBDCh */ case   13:  		/* stb R9, <#[R1 + 8313]> */
		/* 8226CBDCh case   13:*/		cpu::mem::store8( regs, regs.R9, (uint32)(regs.R1 + 0x00002079) );
		/* 8226CBDCh case   13:*/		return 0x8226CBE0;
		  /* 8226CBE0h */ case   14:  		/* mr R6, R28 */
		/* 8226CBE0h case   14:*/		regs.R6 = regs.R28;
		/* 8226CBE0h case   14:*/		return 0x8226CBE4;
		  /* 8226CBE4h */ case   15:  		/* stw R11, <#[R1 + 8296]> */
		/* 8226CBE4h case   15:*/		cpu::mem::store32( regs, regs.R11, (uint32)(regs.R1 + 0x00002068) );
		/* 8226CBE4h case   15:*/		return 0x8226CBE8;
		  /* 8226CBE8h */ case   16:  		/* mr R5, R29 */
		/* 8226CBE8h case   16:*/		regs.R5 = regs.R29;
		/* 8226CBE8h case   16:*/		return 0x8226CBEC;
		  /* 8226CBECh */ case   17:  		/* stw R10, <#[R1 + 8300]> */
		/* 8226CBECh case   17:*/		cpu::mem::store32( regs, regs.R10, (uint32)(regs.R1 + 0x0000206C) );
		/* 8226CBECh case   17:*/		return 0x8226CBF0;
		  /* 8226CBF0h */ case   18:  		/* mr R4, R30 */
		/* 8226CBF0h case   18:*/		regs.R4 = regs.R30;
		/* 8226CBF0h case   18:*/		return 0x8226CBF4;
		  /* 8226CBF4h */ case   19:  		/* addi R3, R1, 80 */
		/* 8226CBF4h case   19:*/		cpu::op::addi<0>(regs,&regs.R3,regs.R1,0x50);
		/* 8226CBF4h case   19:*/		return 0x8226CBF8;
		  /* 8226CBF8h */ case   20:  		/* bl -512 */
		/* 8226CBF8h case   20:*/		regs.LR = 0x8226CBFC; return 0x8226C9F8;
		/* 8226CBF8h case   20:*/		return 0x8226CBFC;
	}
	return 0x8226CBFC;
} // Block from 8226CBA8h-8226CBFCh (21 instructions)

//////////////////////////////////////////////////////
// Block at 8226CBFCh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CBFC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CBFC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CBFC);
		  /* 8226CBFCh */ case    0:  		/* addi R1, R1, 8400 */
		/* 8226CBFCh case    0:*/		cpu::op::addi<0>(regs,&regs.R1,regs.R1,0x20D0);
		/* 8226CBFCh case    0:*/		return 0x8226CC00;
		  /* 8226CC00h */ case    1:  		/* b -1948004 */
		/* 8226CC00h case    1:*/		return 0x8209129C;
		/* 8226CC00h case    1:*/		return 0x8226CC04;
		  /* 8226CC04h */ case    2:  		/* nop */
		/* 8226CC04h case    2:*/		cpu::op::nop();
		/* 8226CC04h case    2:*/		return 0x8226CC08;
	}
	return 0x8226CC08;
} // Block from 8226CBFCh-8226CC08h (3 instructions)

//////////////////////////////////////////////////////
// Block at 8226CC08h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CC08( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CC08) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CC08);
		  /* 8226CC08h */ case    0:  		/* lis R11, -32256 */
		/* 8226CC08h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8200);
		/* 8226CC08h case    0:*/		return 0x8226CC0C;
		  /* 8226CC0Ch */ case    1:  		/* lis R10, -32256 */
		/* 8226CC0Ch case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8200);
		/* 8226CC0Ch case    1:*/		return 0x8226CC10;
		  /* 8226CC10h */ case    2:  		/* addi R11, R11, 16352 */
		/* 8226CC10h case    2:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R11,0x3FE0);
		/* 8226CC10h case    2:*/		return 0x8226CC14;
		  /* 8226CC14h */ case    3:  		/* lis R9, -32217 */
		/* 8226CC14h case    3:*/		cpu::op::lis<0>(regs,&regs.R9,0xFFFF8227);
		/* 8226CC14h case    3:*/		return 0x8226CC18;
		  /* 8226CC18h */ case    4:  		/* addi R9, R9, 5944 */
		/* 8226CC18h case    4:*/		cpu::op::addi<0>(regs,&regs.R9,regs.R9,0x1738);
		/* 8226CC18h case    4:*/		return 0x8226CC1C;
		  /* 8226CC1Ch */ case    5:  		/* lfs FR0, <#[R10 + 2980]> */
		/* 8226CC1Ch case    5:*/		cpu::mem::load32f( regs, &regs.FR0, (uint32)(regs.R10 + 0x00000BA4) );
		/* 8226CC1Ch case    5:*/		return 0x8226CC20;
		  /* 8226CC20h */ case    6:  		/* lfs FR13, <#[R11 + 4]> */
		/* 8226CC20h case    6:*/		cpu::mem::load32f( regs, &regs.FR13, (uint32)(regs.R11 + 0x00000004) );
		/* 8226CC20h case    6:*/		return 0x8226CC24;
		  /* 8226CC24h */ case    7:  		/* lfs FR12, <#[R11 + 8]> */
		/* 8226CC24h case    7:*/		cpu::mem::load32f( regs, &regs.FR12, (uint32)(regs.R11 + 0x00000008) );
		/* 8226CC24h case    7:*/		return 0x8226CC28;
		  /* 8226CC28h */ case    8:  		/* fmuls FR13, FR13, FR0 */
		/* 8226CC28h case    8:*/		cpu::op::fmuls<0>(regs,&regs.FR13,regs.FR13,regs.FR0);
		/* 8226CC28h case    8:*/		return 0x8226CC2C;
		  /* 8226CC2Ch */ case    9:  		/* lfs FR10, <#[R11 + 16]> */
		/* 8226CC2Ch case    9:*/		cpu::mem::load32f( regs, &regs.FR10, (uint32)(regs.R11 + 0x00000010) );
		/* 8226CC2Ch case    9:*/		return 0x8226CC30;
		  /* 8226CC30h */ case   10:  		/* fmuls FR12, FR12, FR0 */
		/* 8226CC30h case   10:*/		cpu::op::fmuls<0>(regs,&regs.FR12,regs.FR12,regs.FR0);
		/* 8226CC30h case   10:*/		return 0x8226CC34;
		  /* 8226CC34h */ case   11:  		/* lfs FR9, <#[R11 + 20]> */
		/* 8226CC34h case   11:*/		cpu::mem::load32f( regs, &regs.FR9, (uint32)(regs.R11 + 0x00000014) );
		/* 8226CC34h case   11:*/		return 0x8226CC38;
		  /* 8226CC38h */ case   12:  		/* fmuls FR10, FR10, FR0 */
		/* 8226CC38h case   12:*/		cpu::op::fmuls<0>(regs,&regs.FR10,regs.FR10,regs.FR0);
		/* 8226CC38h case   12:*/		return 0x8226CC3C;
		  /* 8226CC3Ch */ case   13:  		/* lfs FR8, <#[R11 + 24]> */
		/* 8226CC3Ch case   13:*/		cpu::mem::load32f( regs, &regs.FR8, (uint32)(regs.R11 + 0x00000018) );
		/* 8226CC3Ch case   13:*/		return 0x8226CC40;
		  /* 8226CC40h */ case   14:  		/* fmuls FR9, FR9, FR0 */
		/* 8226CC40h case   14:*/		cpu::op::fmuls<0>(regs,&regs.FR9,regs.FR9,regs.FR0);
		/* 8226CC40h case   14:*/		return 0x8226CC44;
		  /* 8226CC44h */ case   15:  		/* lfs FR6, <#[R11 + 32]> */
		/* 8226CC44h case   15:*/		cpu::mem::load32f( regs, &regs.FR6, (uint32)(regs.R11 + 0x00000020) );
		/* 8226CC44h case   15:*/		return 0x8226CC48;
		  /* 8226CC48h */ case   16:  		/* fmuls FR8, FR8, FR0 */
		/* 8226CC48h case   16:*/		cpu::op::fmuls<0>(regs,&regs.FR8,regs.FR8,regs.FR0);
		/* 8226CC48h case   16:*/		return 0x8226CC4C;
		  /* 8226CC4Ch */ case   17:  		/* lfs FR5, <#[R11 + 36]> */
		/* 8226CC4Ch case   17:*/		cpu::mem::load32f( regs, &regs.FR5, (uint32)(regs.R11 + 0x00000024) );
		/* 8226CC4Ch case   17:*/		return 0x8226CC50;
		  /* 8226CC50h */ case   18:  		/* fmuls FR6, FR6, FR0 */
		/* 8226CC50h case   18:*/		cpu::op::fmuls<0>(regs,&regs.FR6,regs.FR6,regs.FR0);
		/* 8226CC50h case   18:*/		return 0x8226CC54;
		  /* 8226CC54h */ case   19:  		/* lfs FR4, <#[R11 + 40]> */
		/* 8226CC54h case   19:*/		cpu::mem::load32f( regs, &regs.FR4, (uint32)(regs.R11 + 0x00000028) );
		/* 8226CC54h case   19:*/		return 0x8226CC58;
		  /* 8226CC58h */ case   20:  		/* fmuls FR5, FR5, FR0 */
		/* 8226CC58h case   20:*/		cpu::op::fmuls<0>(regs,&regs.FR5,regs.FR5,regs.FR0);
		/* 8226CC58h case   20:*/		return 0x8226CC5C;
		  /* 8226CC5Ch */ case   21:  		/* lfs FR11, <#[R11 + 12]> */
		/* 8226CC5Ch case   21:*/		cpu::mem::load32f( regs, &regs.FR11, (uint32)(regs.R11 + 0x0000000C) );
		/* 8226CC5Ch case   21:*/		return 0x8226CC60;
		  /* 8226CC60h */ case   22:  		/* fmuls FR0, FR4, FR0 */
		/* 8226CC60h case   22:*/		cpu::op::fmuls<0>(regs,&regs.FR0,regs.FR4,regs.FR0);
		/* 8226CC60h case   22:*/		return 0x8226CC64;
		  /* 8226CC64h */ case   23:  		/* lfs FR7, <#[R11 + 28]> */
		/* 8226CC64h case   23:*/		cpu::mem::load32f( regs, &regs.FR7, (uint32)(regs.R11 + 0x0000001C) );
		/* 8226CC64h case   23:*/		return 0x8226CC68;
		  /* 8226CC68h */ case   24:  		/* lfs FR3, <#[R11 + 44]> */
		/* 8226CC68h case   24:*/		cpu::mem::load32f( regs, &regs.FR3, (uint32)(regs.R11 + 0x0000002C) );
		/* 8226CC68h case   24:*/		return 0x8226CC6C;
		  /* 8226CC6Ch */ case   25:  		/* stfs FR13, <#[R9 + 4]> */
		/* 8226CC6Ch case   25:*/		cpu::mem::store32f( regs, regs.FR13, (uint32)(regs.R9 + 0x00000004) );
		/* 8226CC6Ch case   25:*/		return 0x8226CC70;
		  /* 8226CC70h */ case   26:  		/* stfs FR12, <#[R9 + 8]> */
		/* 8226CC70h case   26:*/		cpu::mem::store32f( regs, regs.FR12, (uint32)(regs.R9 + 0x00000008) );
		/* 8226CC70h case   26:*/		return 0x8226CC74;
		  /* 8226CC74h */ case   27:  		/* stfs FR11, <#[R9 + 12]> */
		/* 8226CC74h case   27:*/		cpu::mem::store32f( regs, regs.FR11, (uint32)(regs.R9 + 0x0000000C) );
		/* 8226CC74h case   27:*/		return 0x8226CC78;
		  /* 8226CC78h */ case   28:  		/* stfs FR10, <#[R9 + 16]> */
		/* 8226CC78h case   28:*/		cpu::mem::store32f( regs, regs.FR10, (uint32)(regs.R9 + 0x00000010) );
		/* 8226CC78h case   28:*/		return 0x8226CC7C;
		  /* 8226CC7Ch */ case   29:  		/* stfs FR9, <#[R9 + 20]> */
		/* 8226CC7Ch case   29:*/		cpu::mem::store32f( regs, regs.FR9, (uint32)(regs.R9 + 0x00000014) );
		/* 8226CC7Ch case   29:*/		return 0x8226CC80;
		  /* 8226CC80h */ case   30:  		/* stfs FR8, <#[R9 + 24]> */
		/* 8226CC80h case   30:*/		cpu::mem::store32f( regs, regs.FR8, (uint32)(regs.R9 + 0x00000018) );
		/* 8226CC80h case   30:*/		return 0x8226CC84;
		  /* 8226CC84h */ case   31:  		/* stfs FR7, <#[R9 + 28]> */
		/* 8226CC84h case   31:*/		cpu::mem::store32f( regs, regs.FR7, (uint32)(regs.R9 + 0x0000001C) );
		/* 8226CC84h case   31:*/		return 0x8226CC88;
		  /* 8226CC88h */ case   32:  		/* stfs FR6, <#[R9 + 32]> */
		/* 8226CC88h case   32:*/		cpu::mem::store32f( regs, regs.FR6, (uint32)(regs.R9 + 0x00000020) );
		/* 8226CC88h case   32:*/		return 0x8226CC8C;
		  /* 8226CC8Ch */ case   33:  		/* stfs FR5, <#[R9 + 36]> */
		/* 8226CC8Ch case   33:*/		cpu::mem::store32f( regs, regs.FR5, (uint32)(regs.R9 + 0x00000024) );
		/* 8226CC8Ch case   33:*/		return 0x8226CC90;
		  /* 8226CC90h */ case   34:  		/* stfs FR0, <#[R9 + 40]> */
		/* 8226CC90h case   34:*/		cpu::mem::store32f( regs, regs.FR0, (uint32)(regs.R9 + 0x00000028) );
		/* 8226CC90h case   34:*/		return 0x8226CC94;
		  /* 8226CC94h */ case   35:  		/* stfs FR3, <#[R9 + 44]> */
		/* 8226CC94h case   35:*/		cpu::mem::store32f( regs, regs.FR3, (uint32)(regs.R9 + 0x0000002C) );
		/* 8226CC94h case   35:*/		return 0x8226CC98;
		  /* 8226CC98h */ case   36:  		/* bclr 20, CR0_LT */
		/* 8226CC98h case   36:*/		if ( 1 ) { return (uint32)regs.LR; }
		/* 8226CC98h case   36:*/		return 0x8226CC9C;
	}
	return 0x8226CC9C;
} // Block from 8226CC08h-8226CC9Ch (37 instructions)

//////////////////////////////////////////////////////
// Block at 8226CC9Ch
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CC9C( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CC9C) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CC9C);
		  /* 8226CC9Ch */ case    0:  		/* nop */
		/* 8226CC9Ch case    0:*/		cpu::op::nop();
		/* 8226CC9Ch case    0:*/		return 0x8226CCA0;
	}
	return 0x8226CCA0;
} // Block from 8226CC9Ch-8226CCA0h (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226CCA0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CCA0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CCA0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CCA0);
		  /* 8226CCA0h */ case    0:  		/* lis R11, -32256 */
		/* 8226CCA0h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8200);
		/* 8226CCA0h case    0:*/		return 0x8226CCA4;
		  /* 8226CCA4h */ case    1:  		/* lis R10, -32256 */
		/* 8226CCA4h case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8200);
		/* 8226CCA4h case    1:*/		return 0x8226CCA8;
		  /* 8226CCA8h */ case    2:  		/* addi R11, R11, 16400 */
		/* 8226CCA8h case    2:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R11,0x4010);
		/* 8226CCA8h case    2:*/		return 0x8226CCAC;
		  /* 8226CCACh */ case    3:  		/* lis R9, -32217 */
		/* 8226CCACh case    3:*/		cpu::op::lis<0>(regs,&regs.R9,0xFFFF8227);
		/* 8226CCACh case    3:*/		return 0x8226CCB0;
		  /* 8226CCB0h */ case    4:  		/* addi R9, R9, 5992 */
		/* 8226CCB0h case    4:*/		cpu::op::addi<0>(regs,&regs.R9,regs.R9,0x1768);
		/* 8226CCB0h case    4:*/		return 0x8226CCB4;
		  /* 8226CCB4h */ case    5:  		/* lfs FR0, <#[R10 + 2980]> */
		/* 8226CCB4h case    5:*/		cpu::mem::load32f( regs, &regs.FR0, (uint32)(regs.R10 + 0x00000BA4) );
		/* 8226CCB4h case    5:*/		return 0x8226CCB8;
		  /* 8226CCB8h */ case    6:  		/* lfs FR13, <#[R11 + 4]> */
		/* 8226CCB8h case    6:*/		cpu::mem::load32f( regs, &regs.FR13, (uint32)(regs.R11 + 0x00000004) );
		/* 8226CCB8h case    6:*/		return 0x8226CCBC;
		  /* 8226CCBCh */ case    7:  		/* lfs FR12, <#[R11 + 8]> */
		/* 8226CCBCh case    7:*/		cpu::mem::load32f( regs, &regs.FR12, (uint32)(regs.R11 + 0x00000008) );
		/* 8226CCBCh case    7:*/		return 0x8226CCC0;
		  /* 8226CCC0h */ case    8:  		/* fmuls FR13, FR13, FR0 */
		/* 8226CCC0h case    8:*/		cpu::op::fmuls<0>(regs,&regs.FR13,regs.FR13,regs.FR0);
		/* 8226CCC0h case    8:*/		return 0x8226CCC4;
		  /* 8226CCC4h */ case    9:  		/* lfs FR10, <#[R11 + 16]> */
		/* 8226CCC4h case    9:*/		cpu::mem::load32f( regs, &regs.FR10, (uint32)(regs.R11 + 0x00000010) );
		/* 8226CCC4h case    9:*/		return 0x8226CCC8;
		  /* 8226CCC8h */ case   10:  		/* fmuls FR12, FR12, FR0 */
		/* 8226CCC8h case   10:*/		cpu::op::fmuls<0>(regs,&regs.FR12,regs.FR12,regs.FR0);
		/* 8226CCC8h case   10:*/		return 0x8226CCCC;
		  /* 8226CCCCh */ case   11:  		/* lfs FR9, <#[R11 + 20]> */
		/* 8226CCCCh case   11:*/		cpu::mem::load32f( regs, &regs.FR9, (uint32)(regs.R11 + 0x00000014) );
		/* 8226CCCCh case   11:*/		return 0x8226CCD0;
		  /* 8226CCD0h */ case   12:  		/* fmuls FR10, FR10, FR0 */
		/* 8226CCD0h case   12:*/		cpu::op::fmuls<0>(regs,&regs.FR10,regs.FR10,regs.FR0);
		/* 8226CCD0h case   12:*/		return 0x8226CCD4;
		  /* 8226CCD4h */ case   13:  		/* lfs FR8, <#[R11 + 24]> */
		/* 8226CCD4h case   13:*/		cpu::mem::load32f( regs, &regs.FR8, (uint32)(regs.R11 + 0x00000018) );
		/* 8226CCD4h case   13:*/		return 0x8226CCD8;
		  /* 8226CCD8h */ case   14:  		/* fmuls FR9, FR9, FR0 */
		/* 8226CCD8h case   14:*/		cpu::op::fmuls<0>(regs,&regs.FR9,regs.FR9,regs.FR0);
		/* 8226CCD8h case   14:*/		return 0x8226CCDC;
		  /* 8226CCDCh */ case   15:  		/* lfs FR6, <#[R11 + 32]> */
		/* 8226CCDCh case   15:*/		cpu::mem::load32f( regs, &regs.FR6, (uint32)(regs.R11 + 0x00000020) );
		/* 8226CCDCh case   15:*/		return 0x8226CCE0;
		  /* 8226CCE0h */ case   16:  		/* fmuls FR8, FR8, FR0 */
		/* 8226CCE0h case   16:*/		cpu::op::fmuls<0>(regs,&regs.FR8,regs.FR8,regs.FR0);
		/* 8226CCE0h case   16:*/		return 0x8226CCE4;
		  /* 8226CCE4h */ case   17:  		/* lfs FR5, <#[R11 + 36]> */
		/* 8226CCE4h case   17:*/		cpu::mem::load32f( regs, &regs.FR5, (uint32)(regs.R11 + 0x00000024) );
		/* 8226CCE4h case   17:*/		return 0x8226CCE8;
		  /* 8226CCE8h */ case   18:  		/* fmuls FR6, FR6, FR0 */
		/* 8226CCE8h case   18:*/		cpu::op::fmuls<0>(regs,&regs.FR6,regs.FR6,regs.FR0);
		/* 8226CCE8h case   18:*/		return 0x8226CCEC;
		  /* 8226CCECh */ case   19:  		/* lfs FR4, <#[R11 + 40]> */
		/* 8226CCECh case   19:*/		cpu::mem::load32f( regs, &regs.FR4, (uint32)(regs.R11 + 0x00000028) );
		/* 8226CCECh case   19:*/		return 0x8226CCF0;
		  /* 8226CCF0h */ case   20:  		/* fmuls FR5, FR5, FR0 */
		/* 8226CCF0h case   20:*/		cpu::op::fmuls<0>(regs,&regs.FR5,regs.FR5,regs.FR0);
		/* 8226CCF0h case   20:*/		return 0x8226CCF4;
		  /* 8226CCF4h */ case   21:  		/* lfs FR11, <#[R11 + 12]> */
		/* 8226CCF4h case   21:*/		cpu::mem::load32f( regs, &regs.FR11, (uint32)(regs.R11 + 0x0000000C) );
		/* 8226CCF4h case   21:*/		return 0x8226CCF8;
		  /* 8226CCF8h */ case   22:  		/* fmuls FR0, FR4, FR0 */
		/* 8226CCF8h case   22:*/		cpu::op::fmuls<0>(regs,&regs.FR0,regs.FR4,regs.FR0);
		/* 8226CCF8h case   22:*/		return 0x8226CCFC;
		  /* 8226CCFCh */ case   23:  		/* lfs FR7, <#[R11 + 28]> */
		/* 8226CCFCh case   23:*/		cpu::mem::load32f( regs, &regs.FR7, (uint32)(regs.R11 + 0x0000001C) );
		/* 8226CCFCh case   23:*/		return 0x8226CD00;
		  /* 8226CD00h */ case   24:  		/* lfs FR3, <#[R11 + 44]> */
		/* 8226CD00h case   24:*/		cpu::mem::load32f( regs, &regs.FR3, (uint32)(regs.R11 + 0x0000002C) );
		/* 8226CD00h case   24:*/		return 0x8226CD04;
		  /* 8226CD04h */ case   25:  		/* stfs FR13, <#[R9 + 4]> */
		/* 8226CD04h case   25:*/		cpu::mem::store32f( regs, regs.FR13, (uint32)(regs.R9 + 0x00000004) );
		/* 8226CD04h case   25:*/		return 0x8226CD08;
		  /* 8226CD08h */ case   26:  		/* stfs FR12, <#[R9 + 8]> */
		/* 8226CD08h case   26:*/		cpu::mem::store32f( regs, regs.FR12, (uint32)(regs.R9 + 0x00000008) );
		/* 8226CD08h case   26:*/		return 0x8226CD0C;
		  /* 8226CD0Ch */ case   27:  		/* stfs FR11, <#[R9 + 12]> */
		/* 8226CD0Ch case   27:*/		cpu::mem::store32f( regs, regs.FR11, (uint32)(regs.R9 + 0x0000000C) );
		/* 8226CD0Ch case   27:*/		return 0x8226CD10;
		  /* 8226CD10h */ case   28:  		/* stfs FR10, <#[R9 + 16]> */
		/* 8226CD10h case   28:*/		cpu::mem::store32f( regs, regs.FR10, (uint32)(regs.R9 + 0x00000010) );
		/* 8226CD10h case   28:*/		return 0x8226CD14;
		  /* 8226CD14h */ case   29:  		/* stfs FR9, <#[R9 + 20]> */
		/* 8226CD14h case   29:*/		cpu::mem::store32f( regs, regs.FR9, (uint32)(regs.R9 + 0x00000014) );
		/* 8226CD14h case   29:*/		return 0x8226CD18;
		  /* 8226CD18h */ case   30:  		/* stfs FR8, <#[R9 + 24]> */
		/* 8226CD18h case   30:*/		cpu::mem::store32f( regs, regs.FR8, (uint32)(regs.R9 + 0x00000018) );
		/* 8226CD18h case   30:*/		return 0x8226CD1C;
		  /* 8226CD1Ch */ case   31:  		/* stfs FR7, <#[R9 + 28]> */
		/* 8226CD1Ch case   31:*/		cpu::mem::store32f( regs, regs.FR7, (uint32)(regs.R9 + 0x0000001C) );
		/* 8226CD1Ch case   31:*/		return 0x8226CD20;
		  /* 8226CD20h */ case   32:  		/* stfs FR6, <#[R9 + 32]> */
		/* 8226CD20h case   32:*/		cpu::mem::store32f( regs, regs.FR6, (uint32)(regs.R9 + 0x00000020) );
		/* 8226CD20h case   32:*/		return 0x8226CD24;
		  /* 8226CD24h */ case   33:  		/* stfs FR5, <#[R9 + 36]> */
		/* 8226CD24h case   33:*/		cpu::mem::store32f( regs, regs.FR5, (uint32)(regs.R9 + 0x00000024) );
		/* 8226CD24h case   33:*/		return 0x8226CD28;
		  /* 8226CD28h */ case   34:  		/* stfs FR0, <#[R9 + 40]> */
		/* 8226CD28h case   34:*/		cpu::mem::store32f( regs, regs.FR0, (uint32)(regs.R9 + 0x00000028) );
		/* 8226CD28h case   34:*/		return 0x8226CD2C;
		  /* 8226CD2Ch */ case   35:  		/* stfs FR3, <#[R9 + 44]> */
		/* 8226CD2Ch case   35:*/		cpu::mem::store32f( regs, regs.FR3, (uint32)(regs.R9 + 0x0000002C) );
		/* 8226CD2Ch case   35:*/		return 0x8226CD30;
		  /* 8226CD30h */ case   36:  		/* bclr 20, CR0_LT */
		/* 8226CD30h case   36:*/		if ( 1 ) { return (uint32)regs.LR; }
		/* 8226CD30h case   36:*/		return 0x8226CD34;
	}
	return 0x8226CD34;
} // Block from 8226CCA0h-8226CD34h (37 instructions)

//////////////////////////////////////////////////////
// Block at 8226CD34h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CD34( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CD34) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CD34);
		  /* 8226CD34h */ case    0:  		/* nop */
		/* 8226CD34h case    0:*/		cpu::op::nop();
		/* 8226CD34h case    0:*/		return 0x8226CD38;
	}
	return 0x8226CD38;
} // Block from 8226CD34h-8226CD38h (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226CD38h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CD38( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CD38) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CD38);
		  /* 8226CD38h */ case    0:  		/* lis R11, -32256 */
		/* 8226CD38h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8200);
		/* 8226CD38h case    0:*/		return 0x8226CD3C;
		  /* 8226CD3Ch */ case    1:  		/* lis R10, -32256 */
		/* 8226CD3Ch case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8200);
		/* 8226CD3Ch case    1:*/		return 0x8226CD40;
		  /* 8226CD40h */ case    2:  		/* addi R11, R11, 16352 */
		/* 8226CD40h case    2:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R11,0x3FE0);
		/* 8226CD40h case    2:*/		return 0x8226CD44;
		  /* 8226CD44h */ case    3:  		/* lis R9, -32217 */
		/* 8226CD44h case    3:*/		cpu::op::lis<0>(regs,&regs.R9,0xFFFF8227);
		/* 8226CD44h case    3:*/		return 0x8226CD48;
		  /* 8226CD48h */ case    4:  		/* addi R9, R9, 6040 */
		/* 8226CD48h case    4:*/		cpu::op::addi<0>(regs,&regs.R9,regs.R9,0x1798);
		/* 8226CD48h case    4:*/		return 0x8226CD4C;
		  /* 8226CD4Ch */ case    5:  		/* lfs FR0, <#[R10 + 16872]> */
		/* 8226CD4Ch case    5:*/		cpu::mem::load32f( regs, &regs.FR0, (uint32)(regs.R10 + 0x000041E8) );
		/* 8226CD4Ch case    5:*/		return 0x8226CD50;
		  /* 8226CD50h */ case    6:  		/* lfs FR13, <#[R11 + 4]> */
		/* 8226CD50h case    6:*/		cpu::mem::load32f( regs, &regs.FR13, (uint32)(regs.R11 + 0x00000004) );
		/* 8226CD50h case    6:*/		return 0x8226CD54;
		  /* 8226CD54h */ case    7:  		/* lfs FR12, <#[R11 + 8]> */
		/* 8226CD54h case    7:*/		cpu::mem::load32f( regs, &regs.FR12, (uint32)(regs.R11 + 0x00000008) );
		/* 8226CD54h case    7:*/		return 0x8226CD58;
		  /* 8226CD58h */ case    8:  		/* fmuls FR13, FR13, FR0 */
		/* 8226CD58h case    8:*/		cpu::op::fmuls<0>(regs,&regs.FR13,regs.FR13,regs.FR0);
		/* 8226CD58h case    8:*/		return 0x8226CD5C;
		  /* 8226CD5Ch */ case    9:  		/* lfs FR10, <#[R11 + 16]> */
		/* 8226CD5Ch case    9:*/		cpu::mem::load32f( regs, &regs.FR10, (uint32)(regs.R11 + 0x00000010) );
		/* 8226CD5Ch case    9:*/		return 0x8226CD60;
		  /* 8226CD60h */ case   10:  		/* fmuls FR12, FR12, FR0 */
		/* 8226CD60h case   10:*/		cpu::op::fmuls<0>(regs,&regs.FR12,regs.FR12,regs.FR0);
		/* 8226CD60h case   10:*/		return 0x8226CD64;
		  /* 8226CD64h */ case   11:  		/* lfs FR9, <#[R11 + 20]> */
		/* 8226CD64h case   11:*/		cpu::mem::load32f( regs, &regs.FR9, (uint32)(regs.R11 + 0x00000014) );
		/* 8226CD64h case   11:*/		return 0x8226CD68;
		  /* 8226CD68h */ case   12:  		/* fmuls FR10, FR10, FR0 */
		/* 8226CD68h case   12:*/		cpu::op::fmuls<0>(regs,&regs.FR10,regs.FR10,regs.FR0);
		/* 8226CD68h case   12:*/		return 0x8226CD6C;
		  /* 8226CD6Ch */ case   13:  		/* lfs FR8, <#[R11 + 24]> */
		/* 8226CD6Ch case   13:*/		cpu::mem::load32f( regs, &regs.FR8, (uint32)(regs.R11 + 0x00000018) );
		/* 8226CD6Ch case   13:*/		return 0x8226CD70;
		  /* 8226CD70h */ case   14:  		/* fmuls FR9, FR9, FR0 */
		/* 8226CD70h case   14:*/		cpu::op::fmuls<0>(regs,&regs.FR9,regs.FR9,regs.FR0);
		/* 8226CD70h case   14:*/		return 0x8226CD74;
		  /* 8226CD74h */ case   15:  		/* lfs FR6, <#[R11 + 32]> */
		/* 8226CD74h case   15:*/		cpu::mem::load32f( regs, &regs.FR6, (uint32)(regs.R11 + 0x00000020) );
		/* 8226CD74h case   15:*/		return 0x8226CD78;
		  /* 8226CD78h */ case   16:  		/* fmuls FR8, FR8, FR0 */
		/* 8226CD78h case   16:*/		cpu::op::fmuls<0>(regs,&regs.FR8,regs.FR8,regs.FR0);
		/* 8226CD78h case   16:*/		return 0x8226CD7C;
		  /* 8226CD7Ch */ case   17:  		/* lfs FR5, <#[R11 + 36]> */
		/* 8226CD7Ch case   17:*/		cpu::mem::load32f( regs, &regs.FR5, (uint32)(regs.R11 + 0x00000024) );
		/* 8226CD7Ch case   17:*/		return 0x8226CD80;
		  /* 8226CD80h */ case   18:  		/* fmuls FR6, FR6, FR0 */
		/* 8226CD80h case   18:*/		cpu::op::fmuls<0>(regs,&regs.FR6,regs.FR6,regs.FR0);
		/* 8226CD80h case   18:*/		return 0x8226CD84;
		  /* 8226CD84h */ case   19:  		/* lfs FR4, <#[R11 + 40]> */
		/* 8226CD84h case   19:*/		cpu::mem::load32f( regs, &regs.FR4, (uint32)(regs.R11 + 0x00000028) );
		/* 8226CD84h case   19:*/		return 0x8226CD88;
		  /* 8226CD88h */ case   20:  		/* fmuls FR5, FR5, FR0 */
		/* 8226CD88h case   20:*/		cpu::op::fmuls<0>(regs,&regs.FR5,regs.FR5,regs.FR0);
		/* 8226CD88h case   20:*/		return 0x8226CD8C;
		  /* 8226CD8Ch */ case   21:  		/* lfs FR11, <#[R11 + 12]> */
		/* 8226CD8Ch case   21:*/		cpu::mem::load32f( regs, &regs.FR11, (uint32)(regs.R11 + 0x0000000C) );
		/* 8226CD8Ch case   21:*/		return 0x8226CD90;
		  /* 8226CD90h */ case   22:  		/* fmuls FR0, FR4, FR0 */
		/* 8226CD90h case   22:*/		cpu::op::fmuls<0>(regs,&regs.FR0,regs.FR4,regs.FR0);
		/* 8226CD90h case   22:*/		return 0x8226CD94;
		  /* 8226CD94h */ case   23:  		/* lfs FR7, <#[R11 + 28]> */
		/* 8226CD94h case   23:*/		cpu::mem::load32f( regs, &regs.FR7, (uint32)(regs.R11 + 0x0000001C) );
		/* 8226CD94h case   23:*/		return 0x8226CD98;
		  /* 8226CD98h */ case   24:  		/* lfs FR3, <#[R11 + 44]> */
		/* 8226CD98h case   24:*/		cpu::mem::load32f( regs, &regs.FR3, (uint32)(regs.R11 + 0x0000002C) );
		/* 8226CD98h case   24:*/		return 0x8226CD9C;
		  /* 8226CD9Ch */ case   25:  		/* stfs FR13, <#[R9 + 4]> */
		/* 8226CD9Ch case   25:*/		cpu::mem::store32f( regs, regs.FR13, (uint32)(regs.R9 + 0x00000004) );
		/* 8226CD9Ch case   25:*/		return 0x8226CDA0;
		  /* 8226CDA0h */ case   26:  		/* stfs FR12, <#[R9 + 8]> */
		/* 8226CDA0h case   26:*/		cpu::mem::store32f( regs, regs.FR12, (uint32)(regs.R9 + 0x00000008) );
		/* 8226CDA0h case   26:*/		return 0x8226CDA4;
		  /* 8226CDA4h */ case   27:  		/* stfs FR11, <#[R9 + 12]> */
		/* 8226CDA4h case   27:*/		cpu::mem::store32f( regs, regs.FR11, (uint32)(regs.R9 + 0x0000000C) );
		/* 8226CDA4h case   27:*/		return 0x8226CDA8;
		  /* 8226CDA8h */ case   28:  		/* stfs FR10, <#[R9 + 16]> */
		/* 8226CDA8h case   28:*/		cpu::mem::store32f( regs, regs.FR10, (uint32)(regs.R9 + 0x00000010) );
		/* 8226CDA8h case   28:*/		return 0x8226CDAC;
		  /* 8226CDACh */ case   29:  		/* stfs FR9, <#[R9 + 20]> */
		/* 8226CDACh case   29:*/		cpu::mem::store32f( regs, regs.FR9, (uint32)(regs.R9 + 0x00000014) );
		/* 8226CDACh case   29:*/		return 0x8226CDB0;
		  /* 8226CDB0h */ case   30:  		/* stfs FR8, <#[R9 + 24]> */
		/* 8226CDB0h case   30:*/		cpu::mem::store32f( regs, regs.FR8, (uint32)(regs.R9 + 0x00000018) );
		/* 8226CDB0h case   30:*/		return 0x8226CDB4;
		  /* 8226CDB4h */ case   31:  		/* stfs FR7, <#[R9 + 28]> */
		/* 8226CDB4h case   31:*/		cpu::mem::store32f( regs, regs.FR7, (uint32)(regs.R9 + 0x0000001C) );
		/* 8226CDB4h case   31:*/		return 0x8226CDB8;
		  /* 8226CDB8h */ case   32:  		/* stfs FR6, <#[R9 + 32]> */
		/* 8226CDB8h case   32:*/		cpu::mem::store32f( regs, regs.FR6, (uint32)(regs.R9 + 0x00000020) );
		/* 8226CDB8h case   32:*/		return 0x8226CDBC;
		  /* 8226CDBCh */ case   33:  		/* stfs FR5, <#[R9 + 36]> */
		/* 8226CDBCh case   33:*/		cpu::mem::store32f( regs, regs.FR5, (uint32)(regs.R9 + 0x00000024) );
		/* 8226CDBCh case   33:*/		return 0x8226CDC0;
		  /* 8226CDC0h */ case   34:  		/* stfs FR0, <#[R9 + 40]> */
		/* 8226CDC0h case   34:*/		cpu::mem::store32f( regs, regs.FR0, (uint32)(regs.R9 + 0x00000028) );
		/* 8226CDC0h case   34:*/		return 0x8226CDC4;
		  /* 8226CDC4h */ case   35:  		/* stfs FR3, <#[R9 + 44]> */
		/* 8226CDC4h case   35:*/		cpu::mem::store32f( regs, regs.FR3, (uint32)(regs.R9 + 0x0000002C) );
		/* 8226CDC4h case   35:*/		return 0x8226CDC8;
		  /* 8226CDC8h */ case   36:  		/* bclr 20, CR0_LT */
		/* 8226CDC8h case   36:*/		if ( 1 ) { return (uint32)regs.LR; }
		/* 8226CDC8h case   36:*/		return 0x8226CDCC;
	}
	return 0x8226CDCC;
} // Block from 8226CD38h-8226CDCCh (37 instructions)

//////////////////////////////////////////////////////
// Block at 8226CDCCh
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CDCC( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CDCC) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CDCC);
		  /* 8226CDCCh */ case    0:  		/* nop */
		/* 8226CDCCh case    0:*/		cpu::op::nop();
		/* 8226CDCCh case    0:*/		return 0x8226CDD0;
	}
	return 0x8226CDD0;
} // Block from 8226CDCCh-8226CDD0h (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226CDD0h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CDD0( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CDD0) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CDD0);
		  /* 8226CDD0h */ case    0:  		/* lis R11, -32256 */
		/* 8226CDD0h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8200);
		/* 8226CDD0h case    0:*/		return 0x8226CDD4;
		  /* 8226CDD4h */ case    1:  		/* lis R10, -32256 */
		/* 8226CDD4h case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8200);
		/* 8226CDD4h case    1:*/		return 0x8226CDD8;
		  /* 8226CDD8h */ case    2:  		/* addi R11, R11, 16400 */
		/* 8226CDD8h case    2:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R11,0x4010);
		/* 8226CDD8h case    2:*/		return 0x8226CDDC;
		  /* 8226CDDCh */ case    3:  		/* lis R9, -32217 */
		/* 8226CDDCh case    3:*/		cpu::op::lis<0>(regs,&regs.R9,0xFFFF8227);
		/* 8226CDDCh case    3:*/		return 0x8226CDE0;
		  /* 8226CDE0h */ case    4:  		/* addi R9, R9, 6088 */
		/* 8226CDE0h case    4:*/		cpu::op::addi<0>(regs,&regs.R9,regs.R9,0x17C8);
		/* 8226CDE0h case    4:*/		return 0x8226CDE4;
		  /* 8226CDE4h */ case    5:  		/* lfs FR0, <#[R10 + 16872]> */
		/* 8226CDE4h case    5:*/		cpu::mem::load32f( regs, &regs.FR0, (uint32)(regs.R10 + 0x000041E8) );
		/* 8226CDE4h case    5:*/		return 0x8226CDE8;
		  /* 8226CDE8h */ case    6:  		/* lfs FR13, <#[R11 + 4]> */
		/* 8226CDE8h case    6:*/		cpu::mem::load32f( regs, &regs.FR13, (uint32)(regs.R11 + 0x00000004) );
		/* 8226CDE8h case    6:*/		return 0x8226CDEC;
		  /* 8226CDECh */ case    7:  		/* lfs FR12, <#[R11 + 8]> */
		/* 8226CDECh case    7:*/		cpu::mem::load32f( regs, &regs.FR12, (uint32)(regs.R11 + 0x00000008) );
		/* 8226CDECh case    7:*/		return 0x8226CDF0;
		  /* 8226CDF0h */ case    8:  		/* fmuls FR13, FR13, FR0 */
		/* 8226CDF0h case    8:*/		cpu::op::fmuls<0>(regs,&regs.FR13,regs.FR13,regs.FR0);
		/* 8226CDF0h case    8:*/		return 0x8226CDF4;
		  /* 8226CDF4h */ case    9:  		/* lfs FR10, <#[R11 + 16]> */
		/* 8226CDF4h case    9:*/		cpu::mem::load32f( regs, &regs.FR10, (uint32)(regs.R11 + 0x00000010) );
		/* 8226CDF4h case    9:*/		return 0x8226CDF8;
		  /* 8226CDF8h */ case   10:  		/* fmuls FR12, FR12, FR0 */
		/* 8226CDF8h case   10:*/		cpu::op::fmuls<0>(regs,&regs.FR12,regs.FR12,regs.FR0);
		/* 8226CDF8h case   10:*/		return 0x8226CDFC;
		  /* 8226CDFCh */ case   11:  		/* lfs FR9, <#[R11 + 20]> */
		/* 8226CDFCh case   11:*/		cpu::mem::load32f( regs, &regs.FR9, (uint32)(regs.R11 + 0x00000014) );
		/* 8226CDFCh case   11:*/		return 0x8226CE00;
		  /* 8226CE00h */ case   12:  		/* fmuls FR10, FR10, FR0 */
		/* 8226CE00h case   12:*/		cpu::op::fmuls<0>(regs,&regs.FR10,regs.FR10,regs.FR0);
		/* 8226CE00h case   12:*/		return 0x8226CE04;
		  /* 8226CE04h */ case   13:  		/* lfs FR8, <#[R11 + 24]> */
		/* 8226CE04h case   13:*/		cpu::mem::load32f( regs, &regs.FR8, (uint32)(regs.R11 + 0x00000018) );
		/* 8226CE04h case   13:*/		return 0x8226CE08;
		  /* 8226CE08h */ case   14:  		/* fmuls FR9, FR9, FR0 */
		/* 8226CE08h case   14:*/		cpu::op::fmuls<0>(regs,&regs.FR9,regs.FR9,regs.FR0);
		/* 8226CE08h case   14:*/		return 0x8226CE0C;
		  /* 8226CE0Ch */ case   15:  		/* lfs FR6, <#[R11 + 32]> */
		/* 8226CE0Ch case   15:*/		cpu::mem::load32f( regs, &regs.FR6, (uint32)(regs.R11 + 0x00000020) );
		/* 8226CE0Ch case   15:*/		return 0x8226CE10;
		  /* 8226CE10h */ case   16:  		/* fmuls FR8, FR8, FR0 */
		/* 8226CE10h case   16:*/		cpu::op::fmuls<0>(regs,&regs.FR8,regs.FR8,regs.FR0);
		/* 8226CE10h case   16:*/		return 0x8226CE14;
		  /* 8226CE14h */ case   17:  		/* lfs FR5, <#[R11 + 36]> */
		/* 8226CE14h case   17:*/		cpu::mem::load32f( regs, &regs.FR5, (uint32)(regs.R11 + 0x00000024) );
		/* 8226CE14h case   17:*/		return 0x8226CE18;
		  /* 8226CE18h */ case   18:  		/* fmuls FR6, FR6, FR0 */
		/* 8226CE18h case   18:*/		cpu::op::fmuls<0>(regs,&regs.FR6,regs.FR6,regs.FR0);
		/* 8226CE18h case   18:*/		return 0x8226CE1C;
		  /* 8226CE1Ch */ case   19:  		/* lfs FR4, <#[R11 + 40]> */
		/* 8226CE1Ch case   19:*/		cpu::mem::load32f( regs, &regs.FR4, (uint32)(regs.R11 + 0x00000028) );
		/* 8226CE1Ch case   19:*/		return 0x8226CE20;
		  /* 8226CE20h */ case   20:  		/* fmuls FR5, FR5, FR0 */
		/* 8226CE20h case   20:*/		cpu::op::fmuls<0>(regs,&regs.FR5,regs.FR5,regs.FR0);
		/* 8226CE20h case   20:*/		return 0x8226CE24;
		  /* 8226CE24h */ case   21:  		/* lfs FR11, <#[R11 + 12]> */
		/* 8226CE24h case   21:*/		cpu::mem::load32f( regs, &regs.FR11, (uint32)(regs.R11 + 0x0000000C) );
		/* 8226CE24h case   21:*/		return 0x8226CE28;
		  /* 8226CE28h */ case   22:  		/* fmuls FR0, FR4, FR0 */
		/* 8226CE28h case   22:*/		cpu::op::fmuls<0>(regs,&regs.FR0,regs.FR4,regs.FR0);
		/* 8226CE28h case   22:*/		return 0x8226CE2C;
		  /* 8226CE2Ch */ case   23:  		/* lfs FR7, <#[R11 + 28]> */
		/* 8226CE2Ch case   23:*/		cpu::mem::load32f( regs, &regs.FR7, (uint32)(regs.R11 + 0x0000001C) );
		/* 8226CE2Ch case   23:*/		return 0x8226CE30;
		  /* 8226CE30h */ case   24:  		/* lfs FR3, <#[R11 + 44]> */
		/* 8226CE30h case   24:*/		cpu::mem::load32f( regs, &regs.FR3, (uint32)(regs.R11 + 0x0000002C) );
		/* 8226CE30h case   24:*/		return 0x8226CE34;
		  /* 8226CE34h */ case   25:  		/* stfs FR13, <#[R9 + 4]> */
		/* 8226CE34h case   25:*/		cpu::mem::store32f( regs, regs.FR13, (uint32)(regs.R9 + 0x00000004) );
		/* 8226CE34h case   25:*/		return 0x8226CE38;
		  /* 8226CE38h */ case   26:  		/* stfs FR12, <#[R9 + 8]> */
		/* 8226CE38h case   26:*/		cpu::mem::store32f( regs, regs.FR12, (uint32)(regs.R9 + 0x00000008) );
		/* 8226CE38h case   26:*/		return 0x8226CE3C;
		  /* 8226CE3Ch */ case   27:  		/* stfs FR11, <#[R9 + 12]> */
		/* 8226CE3Ch case   27:*/		cpu::mem::store32f( regs, regs.FR11, (uint32)(regs.R9 + 0x0000000C) );
		/* 8226CE3Ch case   27:*/		return 0x8226CE40;
		  /* 8226CE40h */ case   28:  		/* stfs FR10, <#[R9 + 16]> */
		/* 8226CE40h case   28:*/		cpu::mem::store32f( regs, regs.FR10, (uint32)(regs.R9 + 0x00000010) );
		/* 8226CE40h case   28:*/		return 0x8226CE44;
		  /* 8226CE44h */ case   29:  		/* stfs FR9, <#[R9 + 20]> */
		/* 8226CE44h case   29:*/		cpu::mem::store32f( regs, regs.FR9, (uint32)(regs.R9 + 0x00000014) );
		/* 8226CE44h case   29:*/		return 0x8226CE48;
		  /* 8226CE48h */ case   30:  		/* stfs FR8, <#[R9 + 24]> */
		/* 8226CE48h case   30:*/		cpu::mem::store32f( regs, regs.FR8, (uint32)(regs.R9 + 0x00000018) );
		/* 8226CE48h case   30:*/		return 0x8226CE4C;
		  /* 8226CE4Ch */ case   31:  		/* stfs FR7, <#[R9 + 28]> */
		/* 8226CE4Ch case   31:*/		cpu::mem::store32f( regs, regs.FR7, (uint32)(regs.R9 + 0x0000001C) );
		/* 8226CE4Ch case   31:*/		return 0x8226CE50;
		  /* 8226CE50h */ case   32:  		/* stfs FR6, <#[R9 + 32]> */
		/* 8226CE50h case   32:*/		cpu::mem::store32f( regs, regs.FR6, (uint32)(regs.R9 + 0x00000020) );
		/* 8226CE50h case   32:*/		return 0x8226CE54;
		  /* 8226CE54h */ case   33:  		/* stfs FR5, <#[R9 + 36]> */
		/* 8226CE54h case   33:*/		cpu::mem::store32f( regs, regs.FR5, (uint32)(regs.R9 + 0x00000024) );
		/* 8226CE54h case   33:*/		return 0x8226CE58;
		  /* 8226CE58h */ case   34:  		/* stfs FR0, <#[R9 + 40]> */
		/* 8226CE58h case   34:*/		cpu::mem::store32f( regs, regs.FR0, (uint32)(regs.R9 + 0x00000028) );
		/* 8226CE58h case   34:*/		return 0x8226CE5C;
		  /* 8226CE5Ch */ case   35:  		/* stfs FR3, <#[R9 + 44]> */
		/* 8226CE5Ch case   35:*/		cpu::mem::store32f( regs, regs.FR3, (uint32)(regs.R9 + 0x0000002C) );
		/* 8226CE5Ch case   35:*/		return 0x8226CE60;
		  /* 8226CE60h */ case   36:  		/* bclr 20, CR0_LT */
		/* 8226CE60h case   36:*/		if ( 1 ) { return (uint32)regs.LR; }
		/* 8226CE60h case   36:*/		return 0x8226CE64;
	}
	return 0x8226CE64;
} // Block from 8226CDD0h-8226CE64h (37 instructions)

//////////////////////////////////////////////////////
// Block at 8226CE64h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CE64( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CE64) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CE64);
		  /* 8226CE64h */ case    0:  		/* nop */
		/* 8226CE64h case    0:*/		cpu::op::nop();
		/* 8226CE64h case    0:*/		return 0x8226CE68;
	}
	return 0x8226CE68;
} // Block from 8226CE64h-8226CE68h (1 instructions)

//////////////////////////////////////////////////////
// Block at 8226CE68h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CE68( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CE68) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CE68);
		  /* 8226CE68h */ case    0:  		/* lis R11, -32215 */
		/* 8226CE68h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8229);
		/* 8226CE68h case    0:*/		return 0x8226CE6C;
		  /* 8226CE6Ch */ case    1:  		/* li R4, 114 */
		/* 8226CE6Ch case    1:*/		cpu::op::li<0>(regs,&regs.R4,0x72);
		/* 8226CE6Ch case    1:*/		return 0x8226CE70;
		  /* 8226CE70h */ case    2:  		/* addi R3, R11, -26140 */
		/* 8226CE70h case    2:*/		cpu::op::addi<0>(regs,&regs.R3,regs.R11,0xFFFF99E4);
		/* 8226CE70h case    2:*/		return 0x8226CE74;
		  /* 8226CE74h */ case    3:  		/* b -309956 */
		/* 8226CE74h case    3:*/		return 0x822213B0;
		/* 8226CE74h case    3:*/		return 0x8226CE78;
	}
	return 0x8226CE78;
} // Block from 8226CE68h-8226CE78h (4 instructions)

//////////////////////////////////////////////////////
// Block at 8226CE78h
//////////////////////////////////////////////////////
uint64 __fastcall _code__block8226CE78( uint64 ip, cpu::CpuRegs& regs )
{
	const uint32 local_instr = (uint32)(ip - 0x8226CE78) / 4;
	switch ( local_instr )
	{
		default:	runtime::InvalidAddress(ip, 0x8226CE78);
		  /* 8226CE78h */ case    0:  		/* lis R11, -32215 */
		/* 8226CE78h case    0:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8229);
		/* 8226CE78h case    0:*/		return 0x8226CE7C;
		  /* 8226CE7Ch */ case    1:  		/* lis R10, -32215 */
		/* 8226CE7Ch case    1:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8229);
		/* 8226CE7Ch case    1:*/		return 0x8226CE80;
		  /* 8226CE80h */ case    2:  		/* addi R4, R11, -26140 */
		/* 8226CE80h case    2:*/		cpu::op::addi<0>(regs,&regs.R4,regs.R11,0xFFFF99E4);
		/* 8226CE80h case    2:*/		return 0x8226CE84;
		  /* 8226CE84h */ case    3:  		/* addi R3, R10, -26096 */
		/* 8226CE84h case    3:*/		cpu::op::addi<0>(regs,&regs.R3,regs.R10,0xFFFF9A10);
		/* 8226CE84h case    3:*/		return 0x8226CE88;
		  /* 8226CE88h */ case    4:  		/* b -306648 */
		/* 8226CE88h case    4:*/		return 0x822220B0;
		/* 8226CE88h case    4:*/		return 0x8226CE8C;
		  /* 8226CE8Ch */ case    5:  		/* nop */
		/* 8226CE8Ch case    5:*/		cpu::op::nop();
		/* 8226CE8Ch case    5:*/		return 0x8226CE90;
		  /* 8226CE90h */ case    6:  		/* lis R11, -32215 */
		/* 8226CE90h case    6:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8229);
		/* 8226CE90h case    6:*/		return 0x8226CE94;
		  /* 8226CE94h */ case    7:  		/* addi R3, R11, -31208 */
		/* 8226CE94h case    7:*/		cpu::op::addi<0>(regs,&regs.R3,regs.R11,0xFFFF8618);
		/* 8226CE94h case    7:*/		return 0x8226CE98;
		  /* 8226CE98h */ case    8:  		/* b -1953888 */
		/* 8226CE98h case    8:*/		return 0x8208FE38;
		/* 8226CE98h case    8:*/		return 0x8226CE9C;
		  /* 8226CE9Ch */ case    9:  		/* nop */
		/* 8226CE9Ch case    9:*/		cpu::op::nop();
		/* 8226CE9Ch case    9:*/		return 0x8226CEA0;
		  /* 8226CEA0h */ case   10:  		/* lis R10, -32215 */
		/* 8226CEA0h case   10:*/		cpu::op::lis<0>(regs,&regs.R10,0xFFFF8229);
		/* 8226CEA0h case   10:*/		return 0x8226CEA4;
		  /* 8226CEA4h */ case   11:  		/* lis R11, -32255 */
		/* 8226CEA4h case   11:*/		cpu::op::lis<0>(regs,&regs.R11,0xFFFF8201);
		/* 8226CEA4h case   11:*/		return 0x8226CEA8;
		  /* 8226CEA8h */ case   12:  		/* addi R3, R10, -30708 */
		/* 8226CEA8h case   12:*/		cpu::op::addi<0>(regs,&regs.R3,regs.R10,0xFFFF880C);
		/* 8226CEA8h case   12:*/		return 0x8226CEAC;
		  /* 8226CEACh */ case   13:  		/* addi R11, R11, -24452 */
		/* 8226CEACh case   13:*/		cpu::op::addi<0>(regs,&regs.R11,regs.R11,0xFFFFA07C);
		/* 8226CEACh case   13:*/		return 0x8226CEB0;
		  /* 8226CEB0h */ case   14:  		/* stw R11, <#[R10 - 30708]> */
		/* 8226CEB0h case   14:*/		cpu::mem::store32( regs, regs.R11, (uint32)(regs.R10 + 0xFFFF880C) );
		/* 8226CEB0h case   14:*/		return 0x8226CEB4;
		  /* 8226CEB4h */ case   15:  		/* b -1939740 */
		/* 8226CEB4h case   15:*/		return 0x82093598;
		/* 8226CEB4h case   15:*/		return 0x8226CEB8;
	}
	return 0x8226CEB8;
} // Block from 8226CE78h-8226CEB8h (16 instructions)

