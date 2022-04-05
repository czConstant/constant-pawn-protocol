const RecruitmentProtocol = artifacts.require("RecruitmentProtocol");

describe("RecruitmentProtocol contract", function (accounts) {
    it("RecruitmentProtocol", async function () {
        const recruitmentProtocol = await RecruitmentProtocol.new();
        expect(true).to.equal(true);
    });
});