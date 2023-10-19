// We do not use Microsoft's implementation because its longjmp unwinds
// the stack to support C++ destructors, and the stack frames generated
// by Chez Scheme do not have the required information for this to work
// properly.
// See https://docs.microsoft.com/en-us/cpp/build/x64-calling-convention

.global S_setjmp
.global S_longjmp

S_setjmp:
	// store nonvolatile registers & control words
	mov %rbx,(%rcx)
	mov %rbp,0x8(%rcx)
	mov %rdi,0x10(%rcx)
	mov %rsi,0x18(%rcx)
	mov %rsp,0x20(%rcx)
	mov %r12,0x28(%rcx)
	mov %r13,0x30(%rcx)
	mov %r14,0x38(%rcx)
	mov %r15,0x40(%rcx)
	stmxcsr 0x48(%rcx)
	fnstcw 0x4c(%rcx)
	movdqu %xmm6,0x50(%rcx)
	movdqu %xmm7,0x60(%rcx)
	movdqu %xmm8,0x70(%rcx)
	movdqu %xmm9,0x80(%rcx)
	movdqu %xmm10,0x90(%rcx)
	movdqu %xmm11,0xa0(%rcx)
	movdqu %xmm12,0xb0(%rcx)
	movdqu %xmm13,0xc0(%rcx)
	movdqu %xmm14,0xd0(%rcx)
	movdqu %xmm15,0xe0(%rcx)
	// store return address
	mov (%rsp),%rax
	mov %rax,0xf0(%rcx)
	// return 0
	xor %eax,%eax
	ret

S_longjmp:
	// restore nonvolatile registers & control words
	mov (%rcx),%rbx
	mov 0x8(%rcx),%rbp
	mov 0x10(%rcx),%rdi
	mov 0x18(%rcx),%rsi
	mov 0x20(%rcx),%rsp
	mov 0x28(%rcx),%r12
	mov 0x30(%rcx),%r13
	mov 0x38(%rcx),%r14
	mov 0x40(%rcx),%r15
	ldmxcsr 0x48(%rcx)
	fldcw 0x4c(%rcx)
	movdqu 0x50(%rcx),%xmm6
	movdqu 0x60(%rcx),%xmm7
	movdqu 0x70(%rcx),%xmm8
	movdqu 0x80(%rcx),%xmm9
	movdqu 0x90(%rcx),%xmm10
	movdqu 0xa0(%rcx),%xmm11
	movdqu 0xb0(%rcx),%xmm12
	movdqu 0xc0(%rcx),%xmm13
	movdqu 0xd0(%rcx),%xmm14
	movdqu 0xe0(%rcx),%xmm15
	// restore return address
	mov 0xf0(%rcx),%rax
	mov %rax,(%rsp)
	mov %rdx,%rax
	// return
	ret
