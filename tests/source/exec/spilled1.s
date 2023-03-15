	.text
	.globl main
f:
	pushq %rbp
	movq %rsp, %rbp
	addq $-120, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq %rcx, -32(%rbp)
	movq %r8, -40(%rbp)
	movq %r9, -48(%rbp)
	movq %r10, 6(%rbp)
	movq %r10, 7(%rbp)
	movq $9, -80(%rbp)
	movq -80(%rbp), %rax
	movq $10, -72(%rbp)
	movq -72(%rbp), %rax
	movq $11, -120(%rbp)
	movq -120(%rbp), %r8
	movq $12, -88(%rbp)
	movq -88(%rbp), %rdi
	movq $13, -112(%rbp)
	movq -112(%rbp), %rcx
	movq $14, -104(%rbp)
	movq -104(%rbp), %r9
	movq $15, -96(%rbp)
	movq -96(%rbp), %r8
	movq $65, %rdi
	movq -8(%rbp), %r8
	movq -16(%rbp), %rax
	imulq %rax, %r8
	addq %r8, %rdi
	movq -24(%rbp), %rsi
	movq -32(%rbp), %rcx
	movq %rsi, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	movq -40(%rbp), %rcx
	movq %rsi, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	movq -48(%rbp), %rcx
	movq %rsi, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	subq %rsi, %rdi
	movq -56(%rbp), %rdx
	addq %rdx, %rdi
	movq -64(%rbp), %rdx
	subq %rdx, %rdi
	movq -80(%rbp), %rax
	movq -72(%rbp), %rdx
	cmpq %rdx, %rax
	sete %al
	addq %rax, %rdi
	movq -120(%rbp), %r8
	movq -88(%rbp), %rax
	cmpq %rax, %r8
	setl %r8b
	subq %r8, %rdi
	movq -112(%rbp), %rax
	movq -104(%rbp), %rsi
	cmpq %rsi, %rax
	setle %al
	subq %rax, %rdi
	movq -8(%rbp), %r9
	movq -16(%rbp), %rax
	cmpq %rax, %r9
	setg %r9b
	movq -24(%rbp), %rsi
	cmpq %rsi, %r9
	setg %r9b
	addq %r9, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %rcx
	movq $0, %rsi
	subq %rcx, %rsi
	movq -64(%rbp), %r8
	imulq %r8, %rsi
	addq %rsi, %rdi
	movq -8(%rbp), %rsi
	movq -16(%rbp), %rax
	imulq %rax, %rsi
	movq -24(%rbp), %r9
	movq -96(%rbp), %rax
	cmpq %rax, %r9
	setl %r9b
	imulq %r9, %rsi
	addq %rsi, %rdi
	movq -104(%rbp), %rsi
	movq -112(%rbp), %r8
	movq -80(%rbp), %r9
	addq %r9, %r8
	cmpq %r8, %rsi
	setg %sil
	movq -16(%rbp), %rax
	imulq %rax, %rsi
	movq -16(%rbp), %r9
	movq %rsi, %rax
	cqto
	idivq %rax
	movq %rax, %r9
	movq -16(%rbp), %rax
	imulq %rax, %rsi
	movq -16(%rbp), %r9
	movq %rsi, %rax
	cqto
	idivq %rax
	movq %rax, %r9
	subq %rsi, %rdi
	movq -120(%rbp), %rsi
	movq $0, %r8
	subq %rsi, %r8
	addq %r8, %rdi
	movq -120(%rbp), %r8
	movq -120(%rbp), %rsi
	cmpq %rsi, %r8
	setne %r8b
	movq $0, %rsi
	subq %r8, %rsi
	addq %rsi, %rdi
	movq -32(%rbp), %rcx
	addq %rcx, %rdi
	movq -56(%rbp), %r9
	addq %r9, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq -8(%rbp), %r9
	movq -16(%rbp), %rax
	imulq %rax, %r9
	movq -8(%rbp), %rsi
	movq -16(%rbp), %rax
	movq $2, %r8
	cmpq %r8, %rax
	setne %al
	movq $4, %r8
	imulq %r8, %rax
	cmpq %rax, %rsi
	setl %sil
	imulq %rsi, %r9
	movq -24(%rbp), %rsi
	movq %r9, %rax
	cqto
	idivq %rax
	movq %rax, %rsi
	movq -32(%rbp), %rcx
	imulq %rcx, %r9
	movq -40(%rbp), %r8
	movq %r9, %rax
	cqto
	idivq %rax
	movq %rax, %r8
	movq -48(%rbp), %rcx
	movq %r9, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	addq %r9, %rdi
	movq -56(%rbp), %rsi
	movq -64(%rbp), %rcx
	movq %rsi, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	movq -80(%rbp), %r9
	imulq %r9, %rsi
	addq %rsi, %rdi
	movq -72(%rbp), %rsi
	movq -120(%rbp), %r8
	imulq %r8, %rsi
	movq -88(%rbp), %rcx
	movq %rsi, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	addq %rsi, %rdi
	movq $1, %rcx
	movq -112(%rbp), %r9
	movq %rcx, %rax
	cqto
	idivq %rax
	movq %rax, %r9
	addq %rcx, %rdi
	movq $30, %r9
	movq -104(%rbp), %rcx
	movq %r9, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	addq %r9, %rdi
	movq -56(%rbp), %r9
	movq -104(%rbp), %rcx
	movq -104(%rbp), %rsi
	cmpq %rsi, %rcx
	sete %cl
	movq %r9, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	addq %r9, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-72, %rsp
	movq $1, %rdi
	movq $2, %rsi
	movq $3, %rdx
	movq $4, %rcx
	movq $5, %r8
	movq $6, %r9
	movq $7, -72(%rbp)
	movq $8, %rax
	pushq -72(%rbp)
	pushq %rax
	call f
	addq $16, %rsp
	movq $1, %rdi
	movq $1, %rsi
	movq $1, %rdx
	movq $1, %rcx
	movq $1, %r8
	movq $1, %r9
	movq $1, -64(%rbp)
	movq $1, %rax
	pushq -64(%rbp)
	pushq %rax
	call f
	addq $16, %rsp
	movq $2, %rdi
	movq $2, %rsi
	movq $2, %rdx
	movq $2, %rcx
	movq $2, %r8
	movq $2, %r9
	movq $2, -56(%rbp)
	movq $2, %rax
	pushq -56(%rbp)
	pushq %rax
	call f
	addq $16, %rsp
	movq $8, %rdi
	movq $1, %rsi
	movq $2, %rdx
	movq $3, %rcx
	movq $4, %r8
	movq $5, %r9
	movq $6, -48(%rbp)
	movq $7, %rax
	pushq -48(%rbp)
	pushq %rax
	call f
	addq $16, %rsp
	movq $7, %rdi
	movq $6, %rsi
	movq $5, %rdx
	movq $4, %rcx
	movq $3, %r8
	movq $2, %r9
	movq $1, -40(%rbp)
	movq $8, %rax
	pushq -40(%rbp)
	pushq %rax
	call f
	addq $16, %rsp
	movq $1, %r8
	movq $0, %rdi
	subq %r8, %rdi
	movq $2, %rax
	movq $0, %rsi
	subq %rax, %rsi
	movq $3, %rcx
	movq $0, %rdx
	subq %rcx, %rdx
	movq $4, %r8
	movq $0, %rcx
	subq %r8, %rcx
	movq $5, %r9
	movq $0, %r8
	subq %r9, %r8
	movq $6, %rax
	movq $0, %r9
	subq %rax, %r9
	movq $7, %rax
	movq $0, -32(%rbp)
	subq %rax, -32(%rbp)
	movq $8, -24(%rbp)
	movq $0, %rax
	subq -24(%rbp), %rax
	pushq -32(%rbp)
	pushq %rax
	call f
	addq $16, %rsp
	movq $1, %rdi
	movq $2, %rcx
	movq $0, %rsi
	subq %rcx, %rsi
	movq $3, %rdx
	movq $4, %r9
	movq $0, %rcx
	subq %r9, %rcx
	movq $5, %r8
	movq $6, %rax
	movq $0, %r9
	subq %rax, %r9
	movq $7, -16(%rbp)
	movq $8, -8(%rbp)
	movq $0, %rax
	subq -8(%rbp), %rax
	pushq -16(%rbp)
	pushq %rax
	call f
	addq $16, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

